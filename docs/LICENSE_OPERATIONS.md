Verity Licence Operations Manual
Print this. Hand it to any engineer. They'll be operational in minutes.

VERITY CORE BANKING — LICENCE OPERATIONS MANUAL
Architecture Overview
Component	Location	Purpose
Vendor private key	vendor-keys/vendor-private.pem	Signs all licence keys. Never shared.
Vendor public key	vendor-keys/vendor-public.b64	Embedded in every binary at build time. Verifies signatures.
Licence database	Supabase license_keys table	Stores hashed keys, org names, expiry dates.
Edge Function	Supabase verify-license	Validates keys, serves signed binary downloads.
Binary storage	Supabase Storage verity-binaries bucket	Holds verity-core.bin and verity-gateway.bin.
Download page	verity-core-banking.pages.dev/download	Customer-facing licence input and binary download.
1. One-Time Setup (Already Done — Documented for Reference)
1.1 Generate Vendor Keypair
bash
mkdir -p vendor-keys && cd vendor-keys
openssl genpkey -algorithm ED25519 -out vendor-private.pem
openssl pkey -in vendor-private.pem -pubout -out vendor-public.pem
cat vendor-public.pem | base64 -w0 > vendor-public.b64
cd ..
Store vendor-private.pem offline. Back it up on encrypted media. If lost, all existing licences become unverifiable and new binaries must be built.

1.2 Build Binaries With Public Key Embedded
bash
export VERITY_VENDOR_PUBKEY=$(cat vendor-keys/vendor-public.b64)
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release -p verity -p verity-gateway --target x86_64-unknown-linux-gnu
1.3 Upload Binaries to Supabase Storage
bash
curl -X POST "https://vqkeafelksdtqqgixjow.supabase.co/storage/v1/object/verity-binaries/verity-core.bin" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZxa2VhZmVsa3NkdHFxZ2l4am93Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc3OTY1NTIyMywiZXhwIjoyMDk1MjMxMjIzfQ.dpyXBMr2ssGAPHnArEeC4fod4iseSMMUswVs6ZmZ0-0" \
  -H "Content-Type: application/octet-stream" \
  --data-binary "@target/x86_64-unknown-linux-gnu/release/verity"

curl -X POST "https://vqkeafelksdtqqgixjow.supabase.co/storage/v1/object/verity-binaries/verity-gateway.bin" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZxa2VhZmVsa3NkdHFxZ2l4am93Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc3OTY1NTIyMywiZXhwIjoyMDk1MjMxMjIzfQ.dpyXBMr2ssGAPHnArEeC4fod4iseSMMUswVs6ZmZ0-0" \
  -H "Content-Type: application/octet-stream" \
  --data-binary "@target/x86_64-unknown-linux-gnu/release/verity-gateway"
2. Issuing a New Customer Licence
2.1 Generate the Licence Key
bash
./scripts/generate-license.sh "Customer Organisation Name" 365
First argument: exact organisation name (appears in the key payload)

Second argument: number of days until expiry (365 = 1 year)

Output example:

text
VERITY-eyJvcmciOiJDdXN0b21lciBPcmdhbmlzYXRpb24gTmFtZSIsImlzcyI6IjIwMjYtMDUtMjZUMDA6MDA6MDBaIiwiZXhwIjoiMjAyNy0wNS0yNlQwMDowMDowMFoiLCJmZWF0dXJlcyI6WyJjb3JlIiwicGF5bWVudHMiLCJhZ2VudHMiLCJhdG0iXX0=-c2lnbmF0dXJlX2hlcmU=
Copy the entire VERITY-... string. Send this to your customer.

2.2 Store the Licence in Supabase
bash
./scripts/manage-licenses-supabase.sh add "Customer Organisation Name" 365
This hashes the key, stores it in the license_keys table, and prints the key. The customer can now download binaries.

2.3 Send the Customer These Instructions
text
Your Verity licence key: VERITY-...

1. Go to https://verity-core-banking.pages.dev/download
2. Paste your licence key
3. Select "verity (Core Banking Engine)" and click Download
4. Repeat for "verity-gateway (API Gateway)"
5. Install both on your server:

   sudo ./verity install --license VERITY-...
   sudo ./verity-gateway install --license VERITY-...
   sudo systemctl start verity verity-gateway

Your system is now live. The gateway listens on port 443,
the core engine runs on localhost:9000.
3. Revoking a Licence
Use this if a customer stops paying, has a security breach, or upgrades to a new key.

bash
./scripts/manage-licenses-supabase.sh revoke "VERITY-..."
Important: Revocation only prevents future downloads. The binary does not yet check for revocation at runtime. The customer's running instance will continue to work until their next verity install attempt. For immediate shutdown, you must contact the customer.

4. Listing All Active Licences
bash
./scripts/manage-licenses-supabase.sh list
Returns JSON with all rows: org, expires, created_at. Pipe through jq for readable output:

bash
./scripts/manage-licenses-supabase.sh list | jq .
5. Renewing a Licence
Renewal = issuing a new key with a future expiry date. There is no "extend" operation.

Generate a new key: ./scripts/generate-license.sh "Customer Name" 365

Store it: ./scripts/manage-licenses-supabase.sh add "Customer Name" 365

Revoke the old key: ./scripts/manage-licenses-supabase.sh revoke "OLD-VERITY-..."

Send the customer their new key with instructions to re-run verity install --license NEW-KEY

6. Upgrading a Customer's Tier
Currently, all licences have the same feature set (["core","payments","agents","atm"]). When tier gating is implemented:

Edit scripts/generate-license.sh to accept --tier and --modules flags

Generate the upgraded key with the new tier

Store it in Supabase

Revoke the old key

Customer re-installs with the new key — the binary will unlock additional features

7. Troubleshooting
Problem	Cause	Fix
"Invalid licence key" on download page	Key was typed incorrectly, or was never stored in Supabase, or was revoked	Verify the key exists with list. Re-issue if needed.
"Licence expired" on download page	The expires date has passed	Issue a renewal key.
"Error generating download"	The binary file is missing from Supabase Storage	Re-upload the binary (see Section 1.3).
Customer says install fails with "signature invalid"	The binary was built with a different vendor public key than the one that signed the licence	Rebuild the binary with the correct VERITY_VENDOR_PUBKEY.
Customer says "licence bound to different hardware"	The binary was installed on a different server than the first activation	Issue a new key. Hardware binding is per-machine.
manage-licenses-supabase.sh fails with auth error	The .env file is missing or has wrong credentials	Ensure .env contains SUPABASE_URL, SUPABASE_ANON_KEY, and SUPABASE_SERVICE_ROLE_KEY.
8. Quick-Reference Command Card
bash
# Generate a 1-year licence for a new bank
./scripts/generate-license.sh "Bank Name" 365

# Add it to Supabase so downloads work
./scripts/manage-licenses-supabase.sh add "Bank Name" 365

# Revoke a key
./scripts/manage-licenses-supabase.sh revoke "VERITY-..."

# See all licences
./scripts/manage-licenses-supabase.sh list

# Build and upload new binaries after code changes
export VERITY_VENDOR_PUBKEY=$(cat vendor-keys/vendor-public.b64)
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release -p verity -p verity-gateway --target x86_64-unknown-linux-gnu
# Then run the two curl upload commands from Section 1.3

# Redeploy the download page after changes
wrangler pages deploy web/
9. Important Warnings
Never share vendor-private.pem. It is the root of all trust. Anyone with it can forge valid licences.

Back up the private key offline. Encrypted USB drive in a safe. No cloud storage.

Revocation is not retroactive. Revoking a key stops future downloads but does not disable running instances. Assume any revoked customer still has a working binary.

All licences currently grant the same features. The pricing page defines tiers (Institutional/Professional/Sovereign) but the binary does not yet enforce them. This is the next upgrade priority.

The download page is public. Anyone can attempt to enter a key. Only valid keys in the license_keys table will succeed.

