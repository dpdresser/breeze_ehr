# SovaEHR
MVP of SovaEHR SaaS Product

## Project Structure

```
sovaehr/
├── 📄 README.md
├── 📄 .env
├── 📄 .env.example
├── 📄 .gitignore
├── 📄 Cargo.toml
├── 📄 Cargo.lock
├── 📄 ehr_log.csv
│
├── 🗂️ src/
│   ├── 📄 main.rs
│   ├── 📄 lib.rs
│   ├── 📄 state.rs
│   ├── 🗂️ api/
│   │   ├── 📄 mod.rs
│   │   └── 📄 auth.rs
│   ├── 🗂️ routes/
│   │   ├── 📄 mod.rs
│   │   └── 🗂️ auth/
│   │       ├── 📄 mod.rs
│   │       ├── 📄 guard.rs
│   │       ├── 📄 signin.rs
│   │       ├── 📄 signup.rs
│   │       ├── 📄 retrieve_user_id.rs
│   │       └── 📄 delete_user.rs
│   ├── 🗂️ domain/
│   │   ├── 📄 mod.rs
│   │   ├── 🗂️ error/
│   │   │   ├── 📄 mod.rs
│   │   │   ├── 📄 app_error.rs
│   │   │   └── 📄 http_response.rs
│   │   ├── 🗂️ interfaces/
│   │   │   ├── 📄 mod.rs
│   │   │   └── 📄 auth_service.rs
│   │   └── 🗂️ types/
│   │       ├── 📄 mod.rs
│   │       ├── 📄 email.rs
│   │       └── 📄 password.rs
│   ├── 🗂️ services/
│   │   └── 📄 supabase_auth_service.rs
│   └── 🗂️ utils/
│       ├── 📄 mod.rs
│       ├── 📄 config.rs
│       └── 📄 tracing.rs
│
├── 🗂️ tests/
│   └── 🗂️ auth/
│       ├── 📄 main.rs
│       ├── 📄 helpers.rs
│       ├── 📄 health.rs
│       ├── 📄 signin.rs
│       ├── 📄 signup.rs
│       ├── 📄 retrieve_user_id.rs
│       └── 📄 delete_user.rs
│
├── 🗂️ scripts/
│   ├── 📄 dev-reset.sh
│   └── 📄 test_users_init.sh
│
├── 🗂️ supabase/
│   ├── 📄 config.toml
│   ├── 🗂️ migrations/
│   │   ├── 📄 20250918151818_init_core_practice_schema.sql
│   │   └── 📄 20250918190003_add_audit_log.sql
│   └── 🗂️ seeds/
│       └── 📄 after_users.sql
│
├── 🗂️ target/ *(generated)*
└── 🗂️ .vscode/
    ├── 📄 extensions.json
    └── 📄 settings.json
```

## Tech Stack

- **Backend**: Rust (API server)
- **Database**: Supabase (PostgreSQL + Auth + Real-time)
- **Security**: Row Level Security (RLS) + JWT authentication
- **Testing**: Integration tests with test database

## Quick Start

1. **Setup environment**: Copy `.env.example` to `.env` and configure
2. **Reset database**: `./scripts/dev-reset.sh`
3. **Run server**: `cargo run`
4. **Run tests**: `cargo test`

## Database Schema

Core tables for multi-tenant healthcare practice management:
- `practice_roles` - Role definitions (owner, admin, clinician, etc.)
- `practices` - Healthcare practice entities
- `practice_memberships` - User-to-practice relationships  
- `practice_membership_roles` - Role assignments
- `teams` - Practice teams
- `team_members` - Team membership
- `audit_log` - Complete audit trail

## Development

- `./scripts/dev-reset.sh` - Reset local database with test data
- `./scripts/test_users_init.sh` - Create test users
- Database migrations in `supabase/migrations/`
- All tables have RLS policies for security
