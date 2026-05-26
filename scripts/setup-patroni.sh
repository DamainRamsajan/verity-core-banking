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
