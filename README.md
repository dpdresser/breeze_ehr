# SovaEHR
MVP of SovaEHR SaaS Product

## Project Structure

```
sovaehr/
├── 📄 README.md                    # Project documentation
├── 📄 .env                         # Environment variables (local development)
├── 📄 .gitignore                   # Git ignore rules
├── 📄 Cargo.toml                   # Rust project configuration
├── 📄 Cargo.lock                   # Rust dependency lock file
│
├── 🗂️ src/                         # Rust application source code
│   ├── 📄 main.rs                  # Application entry point
│   ├── 📄 lib.rs                   # Library root
│   ├── 📄 api.rs                   # API routes and handlers
│   ├── 📄 state.rs                 # Application state management
│   ├── 🗂️ domain/                  # Domain logic and types
│   │   ├── 📄 mod.rs               # Domain module exports
│   │   └── 🗂️ error/               # Error handling
│   │       ├── 📄 mod.rs           # Error module exports
│   │       ├── 📄 app_error.rs     # Application error types
│   │       └── 📄 http_response.rs # HTTP error responses
│   └── 🗂️ utils/                   # Utility modules
│       ├── 📄 mod.rs               # Utils module exports
│       ├── 📄 config.rs            # Configuration management
│       └── 📄 tracing.rs           # Logging and tracing setup
│
├── 🗂️ tests/                       # Integration tests
│   └── 🗂️ api/                     # API integration tests
│       ├── 📄 main.rs              # Test runner
│       ├── 📄 helpers.rs           # Test utilities
│       └── 📄 health.rs            # Health endpoint tests
│
├── 🗂️ scripts/                     # Development and deployment scripts
│   ├── 📄 dev-reset.sh             # Development database reset script
│   └── 📄 test_users_init.sh       # Test user creation script
│
├── 🗂️ supabase/                    # Supabase backend configuration
│   ├── 📄 config.toml              # Supabase project configuration
│   ├── 🗂️ migrations/              # Database schema migrations
│   │   ├── 📄 20250918151818_init_core_practice_schema.sql  # Core schema
│   │   └── 📄 20250918190003_add_audit_log.sql             # Audit system
│   └── 🗂️ seeds/                   # Database seed data
│       └── 📄 after_users.sql      # Post-user-creation seed data
│
├── 🗂️ target/                      # Rust build artifacts (generated)
└── 🗂️ .vscode/                     # VS Code workspace configuration
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
