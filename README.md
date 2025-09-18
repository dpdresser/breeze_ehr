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
│   └── 📄 main.rs                  # Main Rust application entry point
│
├── 🗂️ scripts/                     # Development and deployment scripts
│   ├── 📄 dev-reset.sh             # Development database reset script
│   └── 📄 test_users_init.sh       # Test user creation script
│
├── 🗂️ supabase/                    # Supabase backend configuration
│   ├── 📄 config.toml              # Supabase project configuration
│   ├── 🗂️ migrations/              # Database schema migrations
│   │   ├── 📄 20250918151818_init_core_practice_schema.sql  # Core schema setup
│   │   └── 📄 20250918190003_add_audit_log.sql             # Audit logging system
│   └── 🗂️ seeds/                   # Database seed data
│       └── 📄 after_users.sql      # Post-user-creation seed data
│
├── 🗂️ target/                      # Rust build artifacts (generated)
└── 🗂️ .vscode/                     # VS Code workspace configuration
```

## Architecture Overview

### **Backend: Supabase (PostgreSQL + Auth + API)**
- **Database**: PostgreSQL with Row Level Security (RLS)
- **Authentication**: Supabase Auth with JWT tokens
- **API**: Auto-generated REST/GraphQL APIs
- **Real-time**: WebSocket subscriptions for live updates

### **Application: Rust**
- **Framework**: Rust binary application
- **Purpose**: Backend services and business logic
- **Configuration**: Cargo-based dependency management

### **Database Schema**

#### Core Tables:
- `practice_roles` - Role definitions (owner, admin, clinician, etc.)
- `practices` - Healthcare practice/organization entities
- `practice_memberships` - User-to-practice relationships
- `practice_membership_roles` - Role assignments per membership
- `teams` - Practice teams for organization
- `team_members` - Team membership tracking
- `audit_log` - Comprehensive audit trail

#### Security Features:
- **Row Level Security (RLS)** on all tables
- **Hierarchical permissions**: Owners > Admins > Other roles
- **Secure functions** with empty search paths
- **Audit triggers** on all data mutations

### **Development Workflow**

#### Environment Setup:
1. **Environment Variables**: Configure `.env` with Supabase credentials
2. **Database Reset**: Run `./scripts/dev-reset.sh` to reset local DB
3. **Test Data**: Automatically creates test users and practice data

#### Database Management:
- **Migrations**: Version-controlled schema changes in `supabase/migrations/`
- **Seeds**: Test data population in `supabase/seeds/`
- **Local Development**: Supabase CLI for local database instance

### **Key Features**

#### Multi-tenant Architecture:
- **Practice-based isolation**: All data scoped to practices
- **Role-based access control**: Granular permissions per role
- **Team organization**: Flexible team structures within practices

#### Security & Compliance:
- **Audit logging**: Complete trail of all data changes
- **RLS policies**: Database-level access control
- **JWT authentication**: Secure API access
- **Performance optimized**: Efficient auth function calls

#### Development Experience:
- **Type safety**: Rust for backend reliability
- **Database migrations**: Version-controlled schema evolution
- **Automated testing**: Script-based test data setup
- **Local development**: Full Supabase stack locally

## Getting Started

1. **Clone the repository**
2. **Set up environment variables** in `.env`
3. **Install dependencies**: Supabase CLI, Rust toolchain
4. **Run development reset**: `./scripts/dev-reset.sh`
5. **Start development**: Local Supabase instance will be running

## Scripts

- `dev-reset.sh` - Complete development environment reset
- `test_users_init.sh` - Create test users via Supabase Admin API

## Database Schema Highlights

- **Secure by default**: All tables have RLS enabled
- **Performance optimized**: Proper indexing and auth function patterns
- **Audit compliant**: Complete change tracking
- **Multi-role support**: Users can have multiple roles per practice
- **Team flexibility**: Practice-based team organization
