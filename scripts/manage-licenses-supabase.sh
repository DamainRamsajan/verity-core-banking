#!/bin/bash
source .env

ACTION="$1"
case "$ACTION" in
  add)
    ORG="$2"
    DAYS="${3:-90}"
    KEY=$(./scripts/generate-license.sh "$ORG" "$DAYS")
    HASH=$(echo -n "$KEY" | sha256sum | awk '{print $1}')
    EXPIRY=$(date -d "+${DAYS} days" -u +"%Y-%m-%dT%H:%M:%SZ")
    echo "Adding licence for $ORG..."
    curl -s -X POST "${SUPABASE_URL}/rest/v1/license_keys" \
      -H "apikey: ${SUPABASE_SERVICE_ROLE_KEY}" \
      -H "Authorization: Bearer ${SUPABASE_SERVICE_ROLE_KEY}" \
      -H "Content-Type: application/json" \
      -d "{\"hash\":\"$HASH\",\"org\":\"$ORG\",\"expires\":\"$EXPIRY\"}" > /dev/null
    echo "Licence key: $KEY"
    ;;
  revoke)
    KEY="$2"
    HASH=$(echo -n "$KEY" | sha256sum | awk '{print $1}')
    curl -s -X DELETE "${SUPABASE_URL}/rest/v1/license_keys?hash=eq.${HASH}" \
      -H "apikey: ${SUPABASE_SERVICE_ROLE_KEY}" \
      -H "Authorization: Bearer ${SUPABASE_SERVICE_ROLE_KEY}" > /dev/null
    echo "Revoked."
    ;;
  list)
    curl -s "${SUPABASE_URL}/rest/v1/license_keys" \
      -H "apikey: ${SUPABASE_ANON_KEY}" \
      -H "Authorization: Bearer ${SUPABASE_ANON_KEY}"
    echo ""
    ;;
  *)
    echo "Usage: $0 {add <org> [days]|revoke <key>|list}"
    ;;
esac
