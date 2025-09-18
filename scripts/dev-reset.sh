#!/usr/bin/env bash
set -euo pipefail

# Load env (needs SUPABASE_DB_URL, SUPABASE_SERVICE_ROLE_KEY, etc.)
if [ -f .env ]; then export $(grep -v '^#' .env | xargs); fi

echo "1) Resetting local DB..."
supabase db reset

echo "2) Creating Auth users via Admin API..."
bash ./scripts/test_users_init.sh

echo "3) Seeding practice/teams after users..."
# Requires the psql client. Install via brew install libpq && echo 'export PATH="/opt/homebrew/opt/libpq/bin:$PATH"' >> ~/.zshrc
psql "$SUPABASE_DB_URL" -v ON_ERROR_STOP=1 -f supabase/seeds/after_users.sql

echo "Done ✅"