#!/bin/bash

# Use environment variables if set, otherwise load from .env
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_SERVICE_ROLE_KEY" ]; then
    export $(grep -v '^#' .env | xargs)
    # Use production variables
    SUPABASE_URL="$SUPABASE_URL_PROD"
    SUPABASE_SERVICE_ROLE_KEY="$SUPABASE_SERVICE_ROLE_KEY_PROD"
fi

echo "Creating users in: $SUPABASE_URL"

# Owners (2)
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"owner1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"owner2@example.com","password":"Password123!","email_confirm":true}'

# Admin (1), Biller (1), Scheduler (1)
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"admin1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"biller1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"scheduler1@example.com","password":"Password123!","email_confirm":true}'

# Clinical supervisors (2)
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"supervisor1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"supervisor2@example.com","password":"Password123!","email_confirm":true}'

# Clinicians (6 total)
for n in 1 2 3 4 5 6; do
  curl -sS -X POST "$SUPABASE_URL/auth/v1/admin/users" \
    -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" \
    -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" \
    -H "Content-Type: application/json" \
    -d "{\"email\":\"clinician${n}@example.com\",\"password\":\"Password123!\",\"email_confirm\":true}";
done

echo "Production users created successfully!"