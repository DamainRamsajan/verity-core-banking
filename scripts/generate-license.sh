#!/bin/bash
set -e
ORG="$1"
DAYS="${2:-365}"
EXPIRY=$(date -d "+${DAYS} days" -u +"%Y-%m-%dT%H:%M:%SZ")
ISSUED=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Build the licence payload (JSON)
PAYLOAD_JSON=$(jq -n \
  --arg org "$ORG" \
  --arg iss "$ISSUED" \
  --arg exp "$EXPIRY" \
  '{org: $org, iss: $iss, exp: $exp, features: ["core","payments","agents","atm"]}')

# Write the raw JSON to a temporary file (required by openssl for Ed25519)
TMPFILE=$(mktemp)
echo -n "$PAYLOAD_JSON" > "$TMPFILE"

# Sign the raw JSON bytes
SIGNATURE_B64=$(openssl pkeyutl -sign -inkey vendor-keys/vendor-private.pem -in "$TMPFILE" | base64 -w0)
rm -f "$TMPFILE"

# Base64-encode the payload for the licence key
PAYLOAD_B64=$(echo -n "$PAYLOAD_JSON" | base64 -w0)

echo "VERITY-${PAYLOAD_B64}-${SIGNATURE_B64}"
