-- 1) Audit table (no PHI; captures table, op, row ids, who, when)
create table if not exists public.audit_log (
  id            bigserial primary key,
  occurred_at   timestamptz not null default now(),
  table_name    text not null,
  operation     text not null check (operation in ('INSERT','UPDATE','DELETE')),
  actor_user_id uuid,                             -- from auth.uid()
  practice_id   uuid,                             -- if available
  row_id        uuid,                             -- if table has uuid PK
  before_data   jsonb,
  after_data    jsonb
);

-- 2) Helper to read auth.uid() safely even outside PostgREST
create or replace function public.current_auth_uid()
returns uuid
language sql
stable
security definer
set search_path = ''
as $$
  select (nullif(current_setting('request.jwt.claims', true), '')::json->>'sub')::uuid
$$;

-- 3) Generic audit trigger function
create or replace function public.fn_audit_trigger()
returns trigger
language plpgsql
security definer
set search_path = ''
as $$
declare
  v_user uuid := public.current_auth_uid()::uuid;
  v_practice uuid;
  v_row_id uuid;
begin
  -- try to pull practice_id and id from NEW/OLD if present
  if (TG_OP = 'INSERT') then
    -- Check if columns exist by trying to access them in a safe way
    begin
      v_practice := (to_jsonb(NEW) ->> 'practice_id')::uuid;
    exception when others then
      v_practice := null;
    end;
    begin
      v_row_id := (to_jsonb(NEW) ->> 'id')::uuid;
    exception when others then
      v_row_id := null;
    end;
    insert into public.audit_log(table_name, operation, actor_user_id, practice_id, row_id, after_data)
    values (TG_TABLE_NAME, TG_OP, v_user, v_practice, v_row_id, to_jsonb(NEW));
    return NEW;
  elsif (TG_OP = 'UPDATE') then
    begin
      v_practice := (to_jsonb(NEW) ->> 'practice_id')::uuid;
    exception when others then
      v_practice := null;
    end;
    begin
      v_row_id := (to_jsonb(NEW) ->> 'id')::uuid;
    exception when others then
      v_row_id := null;
    end;
    insert into public.audit_log(table_name, operation, actor_user_id, practice_id, row_id, before_data, after_data)
    values (TG_TABLE_NAME, TG_OP, v_user, v_practice, v_row_id, to_jsonb(OLD), to_jsonb(NEW));
    return NEW;
  else -- DELETE
    begin
      v_practice := (to_jsonb(OLD) ->> 'practice_id')::uuid;
    exception when others then
      v_practice := null;
    end;
    begin
      v_row_id := (to_jsonb(OLD) ->> 'id')::uuid;
    exception when others then
      v_row_id := null;
    end;
    insert into public.audit_log(table_name, operation, actor_user_id, practice_id, row_id, before_data)
    values (TG_TABLE_NAME, TG_OP, v_user, v_practice, v_row_id, to_jsonb(OLD));
    return OLD;
  end if;
end
$$;

-- 4) Attach triggers to mutable tables (add more as you add tables)
drop trigger if exists trg_audit_practices on public.practices;
create trigger trg_audit_practices
after insert or update or delete on public.practices
for each row execute function public.fn_audit_trigger();

drop trigger if exists trg_audit_memberships on public.practice_memberships;
create trigger trg_audit_memberships
after insert or update or delete on public.practice_memberships
for each row execute function public.fn_audit_trigger();

drop trigger if exists trg_audit_membership_roles on public.practice_membership_roles;
create trigger trg_audit_membership_roles
after insert or update or delete on public.practice_membership_roles
for each row execute function public.fn_audit_trigger();

drop trigger if exists trg_audit_teams on public.teams;
create trigger trg_audit_teams
after insert or update or delete on public.teams
for each row execute function public.fn_audit_trigger();

drop trigger if exists trg_audit_team_members on public.team_members;
create trigger trg_audit_team_members
after insert or update or delete on public.team_members
for each row execute function public.fn_audit_trigger();

-- 5) RLS: audit_log is readable only by owner/admin of the practice (or NULL practice rows)
alter table public.audit_log enable row level security;

create policy "audit_read_owner_admin"
  on public.audit_log
  for select
  to authenticated
  using (
    practice_id is null
    or private.is_owner_or_admin(practice_id)
  );
