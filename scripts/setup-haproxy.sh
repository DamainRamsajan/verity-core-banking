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
