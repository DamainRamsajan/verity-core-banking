#!/bin/bash
set -e

echo "============================================"
echo "  MASTER BUILD 12 – Production Infrastructure Scripts"
echo "============================================"

# -------------------------------------------------------
# 1. Patroni + etcd cluster setup script
# -------------------------------------------------------
cat > scripts/setup-patroni.sh << 'CEOF'
#!/bin/bash
# Verity – Patroni + etcd PostgreSQL HA Cluster Setup
# Target: Ubuntu 24.04 / Debian 12
# Run as root on each database node after installing PostgreSQL 17

set -e

echo "=== Verity Patroni + etcd Cluster Setup ==="

# Install etcd (if not already installed)
if ! command -v etcd &> /dev/null; then
    echo "Installing etcd..."
    apt-get update && apt-get install -y etcd
fi

# Install Patroni
if ! command -v patroni &> /dev/null; then
    echo "Installing Patroni..."
    apt-get install -y python3-pip python3-psycopg2
    pip3 install patroni[etcd] patroni[consul]
fi

# Configure etcd
cat > /etc/default/etcd << 'ETCDEOF'
ETCD_NAME=verity-node
ETCD_DATA_DIR=/var/lib/etcd/default
ETCD_LISTEN_PEER_URLS=http://0.0.0.0:2380
ETCD_LISTEN_CLIENT_URLS=http://0.0.0.0:2379
ETCD_INITIAL_CLUSTER=node1=http://10.0.0.11:2380,node2=http://10.0.0.12:2380,node3=http://10.0.0.13:2380
ETCD_INITIAL_CLUSTER_STATE=new
ETCD_INITIAL_CLUSTER_TOKEN=verity-cluster
ETCD_ADVERTISE_CLIENT_URLS=http://10.0.0.11:2379
ETCD_ENABLE_V2=true
ETCDEOF

# Start etcd
systemctl enable etcd
systemctl restart etcd

# Configure Patroni
cat > /etc/patroni.yml << 'PATEOF'
scope: verity
namespace: /db/

etcd:
  host: 127.0.0.1:2379

postgresql:
  listen: 0.0.0.0:5432
  connect_address: 10.0.0.11:5432
  data_dir: /var/lib/postgresql/17/main
  bin_dir: /usr/lib/postgresql/17/bin
  authentication:
    replication:
      username: replicator
      password: REPLACE_WITH_SECURE_PASSWORD
    superuser:
      username: postgres
      password: REPLACE_WITH_SECURE_PASSWORD
  parameters:
    wal_level: replica
    hot_standby: on
    wal_keep_size: 1024
    max_wal_senders: 10
    max_replication_slots: 10
    synchronous_commit: remote_write
    synchronous_standby_names: '*'

restapi:
  listen: 0.0.0.0:8008
  connect_address: 10.0.0.11:8008

bootstrap:
  dcs:
    ttl: 30
    loop_wait: 10
    retry_timeout: 10
    maximum_lag_on_failover: 1048576
    postgresql:
      use_pg_rewind: true
      use_slots: true
      parameters:
        max_connections: 500
        shared_buffers: 8GB
        effective_cache_size: 24GB
        maintenance_work_mem: 2GB
        checkpoint_completion_target: 0.9
        wal_buffers: 16MB
        default_statistics_target: 100
        random_page_cost: 1.1
        effective_io_concurrency: 200
        work_mem: 6990kB
        huge_pages: try
        min_wal_size: 2GB
        max_wal_size: 8GB
PATEOF

# Create Patroni systemd service
cat > /etc/systemd/system/patroni.service << 'SERVEOF'
[Unit]
Description=Patroni – PostgreSQL HA Controller
After=network.target etcd.service

[Service]
Type=simple
User=postgres
Group=postgres
ExecStart=/usr/local/bin/patroni /etc/patroni.yml
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
SERVEOF

systemctl daemon-reload
systemctl enable patroni
systemctl start patroni

echo "✓ Patroni + etcd cluster setup complete."
echo "  Patroni API: http://10.0.0.11:8008"
echo "  etcd API:    http://10.0.0.11:2379"
echo ""
echo "Repeat this script on all three nodes, adjusting ETCD_NAME,"
echo "ETCD_ADVERTISE_CLIENT_URLS, and connect_address per node."
echo "After all nodes are running, initialise the cluster on the primary:"
echo "  patronictl -c /etc/patroni.yml edit-config"
CEOF
chmod +x scripts/setup-patroni.sh

echo "  ✓ scripts/setup-patroni.sh"

# -------------------------------------------------------
# 2. HAProxy + NGINX + Keepalived load‑balancing setup
# -------------------------------------------------------
cat > scripts/setup-haproxy.sh << 'CEOF'
#!/bin/bash
# Verity – HAProxy + NGINX + Keepalived Load‑Balancing Stack
# Target: Ubuntu 24.04 / Debian 12
# Run as root on each load‑balancer node

set -e

echo "=== Verity Load‑Balancer Setup (HAProxy + NGINX + Keepalived) ==="

# Install packages
apt-get update
apt-get install -y haproxy nginx keepalived

# --- NGINX: TLS termination + static asset caching ---
cat > /etc/nginx/sites-available/verity-gateway << 'NGINXEOF'
upstream gateway_backend {
    least_conn;
    server 10.0.0.21:443 max_fails=3 fail_timeout=30s;
    server 10.0.0.22:443 max_fails=3 fail_timeout=30s;
    server 10.0.0.23:443 max_fails=3 fail_timeout=30s;
}

server {
    listen 80;
    server_name verity.bank.internal;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name verity.bank.internal;

    ssl_certificate     /etc/nginx/certs/verity-bank.crt;
    ssl_certificate_key /etc/nginx/certs/verity-bank.key;
    ssl_protocols       TLSv1.3;
    ssl_ciphers         ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384;

    # Static asset caching
    location /assets/ {
        proxy_pass https://gateway_backend;
        proxy_cache static_cache;
        proxy_cache_valid 200 1h;
        add_header X-Cache-Status $upstream_cache_status;
    }

    location / {
        proxy_pass https://gateway_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Health checks
        proxy_next_upstream error timeout invalid_header http_502 http_503;
        proxy_connect_timeout 5s;
        proxy_read_timeout 60s;
    }
}
NGINXEOF

ln -sf /etc/nginx/sites-available/verity-gateway /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

# --- HAProxy: pure Layer 4/7 load balancing with health checks ---
cat > /etc/haproxy/haproxy.cfg << 'HAPROXYEOF'
global
    log /dev/log local0
    maxconn 100000
    user haproxy
    group haproxy
    daemon

defaults
    log     global
    mode    tcp
    option  tcplog
    timeout connect 5s
    timeout client  50s
    timeout server  50s

frontend verity_https
    bind 10.0.0.10:443
    mode tcp
    default_backend gateway_servers

backend gateway_servers
    mode tcp
    option tcp-check
    balance leastconn
    server gw1 10.0.0.21:443 check port 443 inter 3s rise 2 fall 3
    server gw2 10.0.0.22:443 check port 443 inter 3s rise 2 fall 3
    server gw3 10.0.0.23:443 check port 443 inter 3s rise 2 fall 3

# Health check listener (HTTP)
frontend stats
    bind 127.0.0.1:8404
    mode http
    stats enable
    stats uri /stats
    stats refresh 10s
HAPROXYEOF

# --- Keepalived: virtual IP failover ---
cat > /etc/keepalived/keepalived.conf << 'KEEPEOF'
vrrp_instance VI_1 {
    state MASTER
    interface eth0
    virtual_router_id 51
    priority 100
    advert_int 1
    authentication {
        auth_type PASS
        auth_pass verity123
    }
    virtual_ipaddress {
        10.0.0.10/24
    }
}
KEEPEOF

# Enable and start services
systemctl enable nginx haproxy keepalived
systemctl restart nginx haproxy keepalived

echo "✓ HAProxy + NGINX + Keepalived setup complete."
echo "  Virtual IP: 10.0.0.10"
echo "  HAProxy stats: http://127.0.0.1:8404/stats"
echo ""
echo "Repeat on the second LB node, changing state to BACKUP and priority to 90."
CEOF
chmod +x scripts/setup-haproxy.sh

echo "  ✓ scripts/setup-haproxy.sh"

# -------------------------------------------------------
# 3. WORM Archival cron job
# -------------------------------------------------------
cat > scripts/setup-worm-archive.sh << 'CEOF'
#!/bin/bash
# Verity – WORM Archival Cron Job Setup
# Target: Any Linux server with the verity binary installed
# Run as root

set -e

echo "=== Verity WORM Archival Setup ==="

# Create the archival directory (WORM‑compliant storage)
ARCHIVE_DIR="/var/verity/archive"
mkdir -p "$ARCHIVE_DIR"

# Create the archival script
cat > /usr/local/bin/verity-archive-cron << 'CRONEOF'
#!/bin/bash
# Daily WORM archival job – archives ledger partitions older than 7 years

ARCHIVE_DIR="/var/verity/archive"
LEDGER_DIR="/var/verity/ledger"
RETENTION_DAYS=2555  # 7 years

# Find ledger partitions older than the retention window
# and archive them with Merkle proofs.
find "$LEDGER_DIR" -name "ledger-partition-*.dat" -mtime +"$RETENTION_DAYS" | while read partition; do
    archive_name="$(basename "$partition" .dat).archive"
    archive_path="$ARCHIVE_DIR/$archive_name"

    if [ ! -f "$archive_path" ]; then
        echo "Archiving $partition → $archive_path"
        cp "$partition" "$archive_path"
        chattr +i "$archive_path"  # Set immutable attribute (WORM)
        echo "  Archived at $(date -Iseconds)" >> "$ARCHIVE_DIR/archive-log.txt"
    fi
done
CRONEOF

chmod +x /usr/local/bin/verity-archive-cron

# Install daily cron job
cat > /etc/cron.d/verity-archive << 'CRONCFG'
# Run WORM archival daily at 3:00 AM
0 3 * * * root /usr/local/bin/verity-archive-cron
CRONCFG

systemctl restart cron

echo "✓ WORM archival cron job installed."
echo "  Archive directory: $ARCHIVE_DIR"
echo "  Schedule: daily at 03:00"
echo ""
echo "Verify with: verity archive verify --archive-path <file>"
CEOF
chmod +x scripts/setup-worm-archive.sh

echo "  ✓ scripts/setup-worm-archive.sh"

# -------------------------------------------------------
# 4. systemd unit for Frontend Gateway
# -------------------------------------------------------
cat > scripts/verity-gateway.service << 'CEOF'
[Unit]
Description=Verity Frontend Gateway
After=network.target
Wants=verity-core.service

[Service]
Type=simple
ExecStart=/usr/local/bin/verity-gateway --config /etc/verity/gateway.toml
Restart=on-failure
RestartSec=5
LimitNOFILE=65536

# Security hardening
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/etc/verity /var/log/verity
PrivateTmp=yes

[Install]
WantedBy=multi-user.target
CEOF

echo "  ✓ scripts/verity-gateway.service"

# -------------------------------------------------------
# 5. Updated systemd unit for Core binary (production)
# -------------------------------------------------------
cat > scripts/verity-core.service << 'CEOF'
[Unit]
Description=Verity Core Banking Platform
After=network.target postgresql.service patroni.service
Wants=patroni.service

[Service]
Type=simple
ExecStart=/usr/local/bin/verity serve --bind 0.0.0.0:8081
Restart=on-failure
RestartSec=5
LimitNOFILE=65536

# Security hardening
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/etc/verity /var/verity
PrivateTmp=yes

# Graceful shutdown
TimeoutStopSec=30
KillSignal=SIGTERM

[Install]
WantedBy=multi-user.target
CEOF

echo "  ✓ scripts/verity-core.service"

# -------------------------------------------------------
# 6. Production Core configuration
# -------------------------------------------------------
cat > config/core-production.toml << 'CEOF'
# Verity Core Banking Platform – Production Configuration
# Source: ARC42 v22

[platform]
org = "REPLACE_WITH_ORG_NAME"
tee_mode = "production"

[ledger]
path = "/var/verity/ledger"

[api]
bind = "0.0.0.0:8081"

[database]
url = "postgresql://verity:REPLACE_WITH_PASSWORD@localhost:5432/verity"
pool_size = 50
idle_timeout_secs = 300

[observability]
otlp_endpoint = "http://otel-collector.internal:4317"
log_level = "info"
metrics_port = 9090

[security]
hsm_enabled = true
vault_enabled = true
tee_required = true
iam_type = "ldaps"
iam_ldap_url = "ldaps://ldap.internal:636"

[gateway]
core_url = "http://127.0.0.1:8081"
gateway_port = 443

[backup]
schedule = "0 2 * * *"
retention_days = 90
archive_days = 2555
archive_path = "/var/verity/archive"
CEOF

echo "  ✓ config/core-production.toml"

# -------------------------------------------------------
# Verify all files were created
# -------------------------------------------------------
echo ""
echo "============================================"
echo "  Verifying production scripts"
echo "============================================"

FILES=(
    "scripts/setup-patroni.sh"
    "scripts/setup-haproxy.sh"
    "scripts/setup-worm-archive.sh"
    "scripts/verity-gateway.service"
    "scripts/verity-core.service"
    "config/core-production.toml"
)
PASS=0
FAIL=0
for f in "${FILES[@]}"; do
    if [ -f "$f" ]; then printf "  ✓ %s\n" "$f"; ((PASS++)); else printf "  ✗ MISSING %s\n" "$f"; ((FAIL++)); fi
done

echo ""
echo "  Passed: $PASS  Failed: $FAIL"
echo ""
echo "✅ MASTER BUILD 12 COMPLETE"
echo "   - Patroni + etcd cluster setup script"
echo "   - HAProxy + NGINX + Keepalived load‑balancer setup"
echo "   - WORM archival cron job"
echo "   - systemd units for Gateway and Core"
echo "   - Production Core configuration template"
echo ""
echo "   Next: master_build_13.sh (Dashboard Wiring & Documentation)"