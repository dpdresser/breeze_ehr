#!/bin/bash
export $(grep -v '^#' .env | xargs)

# Owners (2)
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"owner1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"owner2@example.com","password":"Password123!","email_confirm":true}'

# Admin (1), Biller (1), Scheduler (1)
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"admin1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"biller1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"scheduler1@example.com","password":"Password123!","email_confirm":true}'

# Clinical supervisors (2)
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"supervisor1@example.com","password":"Password123!","email_confirm":true}'
curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" -H "Content-Type: application/json" -d '{"email":"supervisor2@example.com","password":"Password123!","email_confirm":true}'

# Clinicians (6 total)
for n in 1 2 3 4 5 6; do
  curl -sS -X POST "http://127.0.0.1:54321/auth/v1/admin/users" \
    -H "apikey: $SUPABASE_SERVICE_ROLE_KEY" \
    -H "Authorization: Bearer $SUPABASE_SERVICE_ROLE_KEY" \
    -H "Content-Type: application/json" \
    -d "{\"email\":\"clinician${n}@example.com\",\"password\":\"Password123!\",\"email_confirm\":true}";
done
