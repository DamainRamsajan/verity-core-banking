VERITY CORE BANKING – DEPLOYMENT & PRODUCTION MIGRATION GUIDE
1. System Overview
Verity Core Banking is delivered as two statically‑linked, self‑contained binaries:

Binary	Role	Network Binding	License Check
verity	Core banking engine (ledger, BIAN domains, AI agents, Merkle proofs, TLA+ model checking)	127.0.0.1:9000 (localhost only)	Offline Ed25519 signature verification against embedded vendor public key
verity-gateway	API gateway, TLS termination, rate limiting, capability token routing, CORS	0.0.0.0:443 (public interface)	Offline Ed25519 signature verification against embedded vendor public key
Both binaries are obtained from a gated download portal, installed with a single command, and run as systemd services. The core never faces the public internet; all external traffic passes through the gateway.

2. Prerequisites
2.1 Hardware Requirements
CPU: x86_64 with Intel TDX or AMD SEV‑SNP support (for TEE mode). Minimum 8 cores.

RAM: 32 GB (64 GB recommended for FHE workloads).

Disk: 100 GB SSD for OS + ledger event store. Separate partition for /var/lib/verity (data directory).

Network: Static IP address. Outbound internet access required only for licence activation (one‑time) and optional online revocation checks.

2.2 Operating System
Ubuntu 24.04 LTS (recommended) or Rocky Linux 9.

Kernel: 6.8+ (for Intel TDX).

Packages: openssl, curl, jq (for verification scripts).

2.3 TEE Enablement (Recommended)
For Intel TDX:

bash
sudo apt-get install -y intel-tdx-tools
sudo tdx-check
For AMD SEV‑SNP:

bash
sudo apt-get install -y sev-tool
sudo sevctl --status
2.4 Network Configuration
Open port 443/tcp inbound.

The core’s port 9000 must be blocked from external access (local firewall rule).

If using an external load balancer, terminate TLS there and forward to gateway’s port 443.

3. Obtaining the Binaries
3.1 Receive Licence Key
You will receive a licence key string from Verity’s licensing team in this format:

text
VERITY-eyJvcmciOi...-c2lnbmF...
3.2 Download via Web Portal
Navigate to https://verity-core-banking.pages.dev/download.

Paste your licence key into the input field.

Select Binary:

verity (Core Banking Engine) – download first.

verity-gateway (API Gateway) – download second.

Click Download. The portal verifies your licence with the Supabase Edge Function and returns a signed, time‑limited download URL directly from Supabase Storage.

The downloaded files will be named verity-0.1.0.bin and verity-gateway-0.1.0.bin (actual names may include version).

3.3 Verify Checksums (Optional but Recommended)
After download, compare the SHA‑256 hash against the values published in the download portal:

bash
sha256sum verity-*.bin
The correct checksums are displayed on the download page after a successful licence validation.

4. Installation
4.1 Copy Binaries to System Path
bash
sudo cp verity-0.1.0.bin /usr/local/bin/verity
sudo cp verity-gateway-0.1.0.bin /usr/local/bin/verity-gateway
sudo chmod 755 /usr/local/bin/verity /usr/local/bin/verity-gateway
4.2 Install and Activate Licences
Run the install subcommand with your licence key. This performs offline verification using the vendor’s public key embedded in the binary and binds the licence to the machine’s hardware fingerprint (MAC address, hostname, disk ID).

bash
sudo verity install --license "VERITY-eyJvcmciOi...-c2lnbmF..."
sudo verity-gateway install --license "VERITY-eyJvcmciOi...-c2lnbmF..."
If the licence is valid and not expired, the command writes configuration files to /etc/verity/ and exits with code 0. If it fails, it will print a specific error:

Licence signature invalid → The key was tampered with or generated with a different vendor key.

Licence expired → Request a renewal from your Verity account manager.

Licence is bound to different hardware → The key was already activated on another machine. Request a new licence or a hardware migration.

4.3 Directory Structure Created
text
/etc/verity/
├── config.toml          # Core configuration
├── gateway.toml         # Gateway configuration
├── license              # Installed licence key (plain text)
├── machine_id           # Hardware fingerprint
└── data/
    └── ledger.db        # SQLite event store (default; can be Postgres)
5. Configuration
5.1 Core Configuration (/etc/verity/config.toml)
The generated config.toml contains sensible defaults. Review these key sections before starting:

toml
[ledger]
# Use "sqlite" for single-node, "postgres" for HA
storage = "sqlite"
path = "/var/lib/verity/data/ledger.db"

[tee]
enabled = true
mode = "tdx"             # or "sev-snp"

[telemetry]
prometheus_port = 9090
log_level = "info"

[agents]
max_concurrent = 10
For production, switch to PostgreSQL:

toml
[ledger]
storage = "postgres"
database_url = "postgresql://user:pass@host:5432/verity_ledger"
5.2 Gateway Configuration (/etc/verity/gateway.toml)
toml
[server]
bind = "0.0.0.0:443"
tls_cert = "/etc/verity/certs/fullchain.pem"
tls_key = "/etc/verity/certs/privkey.pem"

[proxy]
core_url = "http://127.0.0.1:9000"
timeout_seconds = 30

[rate_limit]
requests_per_second = 1000
burst = 2000
Place your TLS certificate and key in /etc/verity/certs/. If you do not yet have a certificate, the gateway can run on port 8080 (HTTP) for initial testing; change bind to 0.0.0.0:8080 and remove the tls_* lines.

6. Starting the System
6.1 Create systemd Unit Files
/etc/systemd/system/verity.service:

ini
[Unit]
Description=Verity Core Banking Engine
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/verity serve
Restart=always
RestartSec=5
User=verity
Group=verity
Environment="RUST_LOG=info"
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
/etc/systemd/system/verity-gateway.service:

ini
[Unit]
Description=Verity API Gateway
After=verity.service

[Service]
Type=simple
ExecStart=/usr/local/bin/verity-gateway serve
Restart=always
RestartSec=5
User=verity
Group=verity
Environment="RUST_LOG=info"
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
6.2 Create the Runtime User
bash
sudo useradd -r -s /bin/false -d /var/lib/verity verity
sudo mkdir -p /var/lib/verity/data
sudo chown -R verity:verity /var/lib/verity /etc/verity
6.3 Enable and Start Services
bash
sudo systemctl daemon-reload
sudo systemctl enable --now verity verity-gateway
7. Health Checks and Verification
7.1 Check Service Status
bash
sudo systemctl status verity verity-gateway
Both should show active (running).

7.2 Verify Core Liveness
bash
sudo journalctl -u verity -f
Look for the startup banner:

text
[INFO] Verity Core Banking Engine v0.1.0
[INFO] TEE attestation: Intel TDX – PASSED
[INFO] Merkle ledger initialised, root: abc123...
[INFO] Σ entries = 0 (conservation law)
7.3 Test Gateway Health Endpoint
bash
curl -k https://localhost/health
Expected response:

json
{"status":"ok","version":"0.1.0","core":"connected"}
If TLS is not yet configured, test over HTTP:

bash
curl http://localhost:8080/health
7.4 Run Benchmark Suite
The binary includes a built‑in benchmarking tool that tests ledger throughput and Merkle proof generation:

bash
sudo -u verity verity benchmark
Output includes transactions per second, proof generation time, and memory usage.

8. Integration Testing
8.1 Create a Test Account
Via the gateway API (the gateway forwards to core’s internal API):

bash
curl -k -X POST https://localhost/api/accounts \
  -H "Content-Type: application/json" \
  -d '{"account_id":"test-001","currency":"USD"}'
8.2 Post a Transaction
bash
curl -k -X POST https://localhost/api/ledger/transaction \
  -H "Content-Type: application/json" \
  -d '{
    "transaction_id": "tx-001",
    "entries": [
      {"account_id": "test-001", "amount": "100.00", "type": "credit"},
      {"account_id": "test-002", "amount": "-100.00", "type": "debit"}
    ]
  }'
8.3 Verify Conservation Invariant
bash
curl -k https://localhost/api/ledger/health
Expected response:

json
{"conservation":"Σ=0.00","merkle_root":"def456...","entry_count":2}
8.4 Test Agent Operation (if licensed)
bash
curl -k -X POST https://localhost/api/agents/action \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <capability_token>" \
  -d '{"agent_id":"agent-001","action":"balance_inquiry","params":{"account":"test-001"}}'
8.5 Simulate Failure and Recovery
Kill the core process and verify automatic restart:

bash
sudo systemctl kill -s SIGKILL verity
sleep 5
sudo systemctl status verity   # Should be running again
9. Migration to Production
When testing is complete, perform these steps to transition from test to production:

9.1 Purge Test Data
If you used test accounts and transactions, reset the ledger:

bash
sudo systemctl stop verity verity-gateway
sudo rm -f /var/lib/verity/data/ledger.db
sudo systemctl start verity verity-gateway
The core will recreate an empty ledger with a new Merkle root.

9.2 Apply Production Licence
If you tested with a trial licence, re‑install with the production key:

bash
sudo verity install --license "VERITY-PRODUCTION-KEY..."
sudo verity-gateway install --license "VERITY-PRODUCTION-KEY..."
sudo systemctl restart verity verity-gateway
The new licence will bind to the machine; no data loss occurs because the licence is stored independently of the ledger.

9.3 Configure TLS with Valid Certificate
Obtain a certificate (e.g., via Let’s Encrypt) and place it in /etc/verity/certs/. Then update gateway.toml:

toml
[server]
tls_cert = "/etc/verity/certs/fullchain.pem"
tls_key = "/etc/verity/certs/privkey.pem"
bind = "0.0.0.0:443"
Restart gateway:

bash
sudo systemctl restart verity-gateway
9.4 Enable Monitoring
Set up Prometheus to scrape :9090/metrics from the core. The gateway’s metrics are exposed on the same port if configured. Add firewall rule to allow monitoring server access to port 9090.

9.5 Backup Configuration
Regularly back up:

/etc/verity/ (all config files and licence)

/var/lib/verity/data/ (ledger database)

TLS certificate and key

Example daily cron job:

bash
0 2 * * * tar -czf /backup/verity-$(date +\%Y\%m\%d).tgz /etc/verity /var/lib/verity/data
10. Maintenance
10.1 Upgrading to a New Version
Download the new binaries via the portal (using your licence key).

Stop services: sudo systemctl stop verity verity-gateway

Replace binaries: sudo cp verity-*.bin /usr/local/bin/verity and similarly for gateway.

Re‑run the install command (to re‑validate licence): sudo verity install --license "$(cat /etc/verity/license)"

Restart: sudo systemctl start verity verity-gateway

The ledger database is automatically migrated by the new binary on first run.

10.2 Monitoring Logs
Core: journalctl -u verity -f

Gateway: journalctl -u verity-gateway -f

Audit events (licence checks, configuration changes) are written to syslog.

10.3 Renewing Licence
When your licence nears expiry, request a new key. Install it as described in Section 9.2. The system does not require a restart; the new licence takes effect immediately.

11. Troubleshooting
Problem	Diagnosis	Solution
Gateway returns 502 Bad Gateway	Core is not running or not listening on 9000	sudo systemctl status verity, check logs for errors.
verity install fails with Permission denied	Not running as root	Use sudo.
verity install hangs	Network timeout trying to check revocation? Actually offline, so this shouldn't happen. If it hangs, check disk space or hardware clock.	Verify system clock is correct (timedatectl status).
Core reports Conservation violation Σ≠0	Corrupted ledger or bug	Restore from backup, contact support.
Gateway logs Rate limit exceeded	Too many requests	Adjust rate_limit in gateway.toml or implement client‑side backoff.
TEE attestation fails	TDX/SEV‑SNP not properly configured	Check BIOS settings, run tdx-check again, ensure kernel supports it.
Licence key stops working on download portal	Key revoked or expired	Contact your account manager for a new key.
12. Quick Reference Card
bash
# Install
sudo cp verity-*.bin /usr/local/bin/verity
sudo verity install --license "VERITY-..."

# Start
sudo systemctl enable --now verity verity-gateway

# Status
sudo systemctl status verity verity-gateway

# Health
curl -k https://localhost/health

# Upgrade
sudo systemctl stop verity verity-gateway
sudo cp new-verity /usr/local/bin/verity
sudo verity install --license "$(cat /etc/verity/license)"
sudo systemctl start verity verity-gateway

# Backup
tar -czf verity-backup.tgz /etc/verity /var/lib/verity/data
This document is accurate to the live system as of build v0.1.0. For any deviations or version‑specific changes, refer to the changelog bundled with the binary.