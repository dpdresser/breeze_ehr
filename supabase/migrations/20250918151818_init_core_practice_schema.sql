-- ===== Lookup: roles =====
create table if not exists public.practice_roles (
  id      uuid primary key default gen_random_uuid(),
  code    text not null unique check (code ~ '^[a-z_]+$'),
  label   text not null
);

insert into public.practice_roles (id, code, label) values
  (gen_random_uuid(), 'owner',     'Owner'),
  (gen_random_uuid(), 'admin',     'Admin'),
  (gen_random_uuid(), 'biller',    'Biller'),
  (gen_random_uuid(), 'scheduler', 'Scheduler'),
  (gen_random_uuid(), 'clinical_supervisor', 'Clinical Supervisor'),
  (gen_random_uuid(), 'clinician', 'Clinician')
on conflict (code) do nothing;

-- ===== Core: practices =====
create table if not exists public.practices (
  id          uuid primary key default gen_random_uuid(),
  name        text not null,
  created_at  timestamptz not null default now(),
  updated_at  timestamptz not null default now(),
  deleted_at  timestamptz
);

create index if not exists idx_practices_name on public.practices (lower(name));

-- ===== Membership (no role here) =====
-- Link identities to practices. Roles live in the join table below.
create table if not exists public.practice_memberships (
  id           uuid primary key default gen_random_uuid(),
  user_id      uuid not null references auth.users(id) on delete cascade,
  practice_id  uuid not null references public.practices(id) on delete cascade,
  is_active    boolean not null default true,
  created_at   timestamptz not null default now(),
  unique (user_id, practice_id)
);

create index if not exists idx_memberships_user on public.practice_memberships (user_id);
create index if not exists idx_memberships_practice on public.practice_memberships (practice_id);

-- ===== Membership roles (many-to-many per membership) =====
create table if not exists public.practice_membership_roles (
  id                 uuid primary key default gen_random_uuid(),
  membership_id      uuid not null references public.practice_memberships(id) on delete cascade,
  role_id            uuid not null references public.practice_roles(id) on delete restrict,
  created_at         timestamptz not null default now(),
  unique (membership_id, role_id)
);

create index if not exists idx_mrole_membership on public.practice_membership_roles (membership_id);
create index if not exists idx_mrole_role on public.practice_membership_roles (role_id);

-- ===== Teams =====
create table if not exists public.teams (
  id           uuid primary key default gen_random_uuid(),
  practice_id  uuid not null references public.practices(id) on delete cascade,
  name         text not null,
  created_at   timestamptz not null default now()
);

create index if not exists idx_teams_practice on public.teams (practice_id);
create unique index if not exists idx_teams_name_unique on public.teams (practice_id, lower(name));

-- ===== Team members =====
create table if not exists public.team_members (
  id           uuid primary key default gen_random_uuid(),
  team_id      uuid not null references public.teams(id) on delete cascade,
  user_id      uuid not null references auth.users(id) on delete cascade,
  practice_id  uuid not null references public.practices(id) on delete cascade,
  created_at   timestamptz not null default now(),
  unique (team_id, user_id)
);

create index if not exists idx_team_members_team on public.team_members (team_id);
create index if not exists idx_team_members_practice on public.team_members (practice_id);

-- ===== RLS on all tables =====
alter table public.practices                   enable row level security;
alter table public.practice_roles              enable row level security;
alter table public.practice_memberships        enable row level security;
alter table public.practice_membership_roles   enable row level security;
alter table public.teams                       enable row level security;
alter table public.team_members                enable row level security;

-- ===== Helper functions (SECURITY DEFINER) =====
create schema if not exists private;

-- Is current user a member of given practice?
create or replace function private.is_member_of_practice(p_practice_id uuid)
returns boolean
language sql
security definer
set search_path = public
as $$
  select exists (
    select 1
    from public.practice_memberships m
    where m.practice_id = p_practice_id
      and m.user_id = (select auth.uid())
      and m.is_active
  );
$$;

-- Does current user have a particular role in a given practice?
create or replace function private.has_role(p_practice_id uuid, p_role_code text)
returns boolean
language sql
security definer
set search_path = public
as $$
  select exists (
    select 1
    from public.practice_memberships m
    join public.practice_membership_roles mr on mr.membership_id = m.id
    join public.practice_roles r on r.id = mr.role_id
    where m.practice_id = p_practice_id
      and m.user_id = (select auth.uid())
      and m.is_active
      and r.code = p_role_code
  );
$$;

-- Owner or Admin convenience
create or replace function private.is_owner_or_admin(p_practice_id uuid)
returns boolean
language sql
security definer
set search_path = public
as $$
  select
    private.has_role(p_practice_id, 'owner')
    or private.has_role(p_practice_id, 'admin');
$$;

comment on function private.is_member_of_practice is 'Checks current auth.uid() is active member of practice';
comment on function private.has_role is 'Checks current auth.uid() has a specific role in practice';
comment on function private.is_owner_or_admin is 'Checks current auth.uid() is owner or admin in practice';

-- ===== POLICIES =====

-- Roles lookup: readable by any authenticated user; no writes from clients
create policy "roles_read_all_auth"
  on public.practice_roles
  for select
  to authenticated
  using (true);
revoke insert, update, delete on public.practice_roles from authenticated;

-- Practices: members can read; owner/admin can write
create policy "practices_select_for_members"
  on public.practices
  for select
  to authenticated
  using (private.is_member_of_practice(id));

create policy "practices_modify_owner_admin"
  on public.practices
  for all
  to authenticated
  using (private.is_owner_or_admin(id))
  with check (private.is_owner_or_admin(id));

-- Memberships: user can read their rows; owner/admin can manage within same practice
create policy "memberships_select_self_or_admin"
  on public.practice_memberships
  for select
  to authenticated
  using (
    user_id = (select auth.uid())
    or private.is_owner_or_admin(practice_id)
  );

create policy "memberships_modify_owner_admin"
  on public.practice_memberships
  for all
  to authenticated
  using (private.is_owner_or_admin(practice_id))
  with check (private.is_owner_or_admin(practice_id));

-- Membership roles: visible to members of the practice; writable by owner/admin
create policy "mroles_select_for_members"
  on public.practice_membership_roles
  for select
  to authenticated
  using (
    exists (
      select 1
      from public.practice_memberships m
      where m.id = membership_id
        and private.is_member_of_practice(m.practice_id)
    )
  );

create policy "mroles_modify_owner_admin"
  on public.practice_membership_roles
  for all
  to authenticated
  using (
    exists (
      select 1
      from public.practice_memberships m
      where m.id = membership_id
        and private.is_owner_or_admin(m.practice_id)
    )
  )
  with check (
    exists (
      select 1
      from public.practice_memberships m
      where m.id = membership_id
        and private.is_owner_or_admin(m.practice_id)
    )
  );

-- Teams: members can read; owner/admin can write
create policy "teams_select_for_members"
  on public.teams
  for select
  to authenticated
  using (private.is_member_of_practice(practice_id));

create policy "teams_modify_owner_admin"
  on public.teams
  for all
  to authenticated
  using (private.is_owner_or_admin(practice_id))
  with check (private.is_owner_or_admin(practice_id));

-- Team members: members can read; owner/admin can write
create policy "team_members_select_for_members"
  on public.team_members
  for select
  to authenticated
  using (private.is_member_of_practice(practice_id));

create policy "team_members_modify_owner_admin"
  on public.team_members
  for all
  to authenticated
  using (private.is_owner_or_admin(practice_id))
  with check (private.is_owner_or_admin(practice_id));