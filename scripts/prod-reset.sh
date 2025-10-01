#!/usr/bin/env bash
set -euo pipefail

# Load production environment variables
if [ -f .env ]; then 
    export $(grep -v '^#' .env | xargs)
else
    echo "Error: .env file not found"
    exit 1
fi

echo "⚠️  WARNING: This will reset your PRODUCTION database!"
echo "Project: $SUPABASE_URL_PROD"
read -p "Are you sure you want to continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "Aborted."
    exit 0
fi

echo "1) Resetting remote DB..."
supabase db reset --linked

echo "2) Creating Auth users via Admin API..."
# Use production environment for user creation
SUPABASE_URL="$SUPABASE_URL_PROD" SUPABASE_SERVICE_ROLE_KEY="$SUPABASE_SERVICE_ROLE_KEY_PROD" \
bash ./scripts/prod_users_init.sh

echo "3) Seeding practice/teams after users..."
# Use production database URL
psql "$SUPABASE_DB_URL_PROD" -v ON_ERROR_STOP=1 -f supabase/seeds/after_users.sql

echo "Production reset complete ✅"