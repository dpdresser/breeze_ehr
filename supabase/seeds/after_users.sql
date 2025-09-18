-- 1) Ensure the practice exists (no unique needed on name; use NOT EXISTS)
insert into public.practices (id, name)
select gen_random_uuid(), 'Test Therapy Practice'
where not exists (
  select 1 from public.practices where name = 'Test Therapy Practice'
);

-- 2) Create memberships for all users in this practice
with u as (
  select
    (select id from auth.users where email = 'owner1@example.com')      as owner1,
    (select id from auth.users where email = 'owner2@example.com')      as owner2,
    (select id from auth.users where email = 'admin1@example.com')      as admin1,
    (select id from auth.users where email = 'biller1@example.com')     as biller1,
    (select id from auth.users where email = 'scheduler1@example.com')  as scheduler1,
    (select id from auth.users where email = 'supervisor1@example.com') as sup1,
    (select id from auth.users where email = 'supervisor2@example.com') as sup2,
    (select id from auth.users where email = 'clinician1@example.com')  as c1,
    (select id from auth.users where email = 'clinician2@example.com')  as c2,
    (select id from auth.users where email = 'clinician3@example.com')  as c3,
    (select id from auth.users where email = 'clinician4@example.com')  as c4,
    (select id from auth.users where email = 'clinician5@example.com')  as c5,
    (select id from auth.users where email = 'clinician6@example.com')  as c6
),
p as (
  select id as practice_id
  from public.practices
  where name = 'Test Therapy Practice'
)
insert into public.practice_memberships (user_id, practice_id)
select uid, p.practice_id
from p
cross join u
cross join unnest(array[u.owner1, u.owner2, u.admin1, u.biller1, u.scheduler1,
                       u.sup1, u.sup2, u.c1, u.c2, u.c3, u.c4, u.c5, u.c6]) as uid
where uid is not null
on conflict (user_id, practice_id) do nothing;

-- 3) Attach roles to memberships (multi-role model)
-- owners
with u as (
  select
    (select id from auth.users where email = 'owner1@example.com') as u1,
    (select id from auth.users where email = 'owner2@example.com') as u2
),
p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
r as (select id as role_id from public.practice_roles where code = 'owner')
insert into public.practice_membership_roles (membership_id, role_id)
select m.id, r.role_id
from p
join public.practice_memberships m on m.practice_id = p.practice_id
join r on true
join u on m.user_id in (u.u1, u.u2)
on conflict (membership_id, role_id) do nothing;

-- admin
with u as (
  select (select id from auth.users where email = 'admin1@example.com') as u1
),
p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
r as (select id as role_id from public.practice_roles where code = 'admin')
insert into public.practice_membership_roles (membership_id, role_id)
select m.id, r.role_id
from p
join public.practice_memberships m on m.practice_id = p.practice_id and m.user_id = (select u1 from u)
join r on true
on conflict (membership_id, role_id) do nothing;

-- biller
with u as (
  select (select id from auth.users where email = 'biller1@example.com') as u1
),
p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
r as (select id as role_id from public.practice_roles where code = 'biller')
insert into public.practice_membership_roles (membership_id, role_id)
select m.id, r.role_id
from p
join public.practice_memberships m on m.practice_id = p.practice_id and m.user_id = (select u1 from u)
join r on true
on conflict (membership_id, role_id) do nothing;

-- scheduler
with u as (
  select (select id from auth.users where email = 'scheduler1@example.com') as u1
),
p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
r as (select id as role_id from public.practice_roles where code = 'scheduler')
insert into public.practice_membership_roles (membership_id, role_id)
select m.id, r.role_id
from p
join public.practice_memberships m on m.practice_id = p.practice_id and m.user_id = (select u1 from u)
join r on true
on conflict (membership_id, role_id) do nothing;

-- clinical supervisors (2)
with u as (
  select
    (select id from auth.users where email = 'supervisor1@example.com') as u1,
    (select id from auth.users where email = 'supervisor2@example.com') as u2
),
p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
r as (select id as role_id from public.practice_roles where code = 'clinical_supervisor')
insert into public.practice_membership_roles (membership_id, role_id)
select m.id, r.role_id
from p
join public.practice_memberships m on m.practice_id = p.practice_id
join r on true
join u on m.user_id in (u.u1, u.u2)
on conflict (membership_id, role_id) do nothing;

-- clinicians (6)
with u as (
  select
    (select id from auth.users where email = 'clinician1@example.com') as u1,
    (select id from auth.users where email = 'clinician2@example.com') as u2,
    (select id from auth.users where email = 'clinician3@example.com') as u3,
    (select id from auth.users where email = 'clinician4@example.com') as u4,
    (select id from auth.users where email = 'clinician5@example.com') as u5,
    (select id from auth.users where email = 'clinician6@example.com') as u6
),
p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
r as (select id as role_id from public.practice_roles where code = 'clinician')
insert into public.practice_membership_roles (membership_id, role_id)
select m.id, r.role_id
from p
join public.practice_memberships m on m.practice_id = p.practice_id
join r on true
join u on m.user_id in (u.u1, u.u2, u.u3, u.u4, u.u5, u.u6)
on conflict (membership_id, role_id) do nothing;

-- 4) Create teams (uses unique index on (practice_id, lower(name)) from schema step)
with p as (select id as practice_id from public.practices where name = 'Test Therapy Practice')
insert into public.teams (practice_id, name)
select p.practice_id, v.tname
from p,
(values ('Operations'::text),
        ('Supervision Alpha'::text),
        ('Supervision Beta'::text)) as v(tname)
on conflict (practice_id, lower(name)) do nothing;

-- 5) Add team members

-- Operations: owners + admin + biller + scheduler
with p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
t as (select id as team_id, practice_id from public.teams where name = 'Operations' and practice_id = (select practice_id from p)),
u as (
  select
    (select id from auth.users where email = 'owner1@example.com') as u1,
    (select id from auth.users where email = 'owner2@example.com') as u2,
    (select id from auth.users where email = 'admin1@example.com') as u3,
    (select id from auth.users where email = 'biller1@example.com') as u4,
    (select id from auth.users where email = 'scheduler1@example.com') as u5
)
insert into public.team_members (team_id, user_id, practice_id)
select t.team_id, uid, t.practice_id
from t
cross join u
cross join unnest(array[u.u1, u.u2, u.u3, u.u4, u.u5]) as uid
where uid is not null
on conflict (team_id, user_id) do nothing;

-- Supervision Alpha: 1 supervisor + 2 clinicians (sup1, c1, c2)
with p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
t as (select id as team_id, practice_id from public.teams where name = 'Supervision Alpha' and practice_id = (select practice_id from p)),
u as (
  select
    (select id from auth.users where email = 'supervisor1@example.com') as u1,
    (select id from auth.users where email = 'clinician1@example.com')  as u2,
    (select id from auth.users where email = 'clinician2@example.com')  as u3
)
insert into public.team_members (team_id, user_id, practice_id)
select t.team_id, uid, t.practice_id
from t
cross join u
cross join unnest(array[u.u1, u.u2, u.u3]) as uid
where uid is not null
on conflict (team_id, user_id) do nothing;

-- Supervision Beta: 2 supervisors + 4 clinicians (sup1, sup2, c3..c6)
with p as (select id as practice_id from public.practices where name = 'Test Therapy Practice'),
t as (select id as team_id, practice_id from public.teams where name = 'Supervision Beta' and practice_id = (select practice_id from p)),
u as (
  select
    (select id from auth.users where email = 'supervisor1@example.com') as u1,
    (select id from auth.users where email = 'supervisor2@example.com') as u2,
    (select id from auth.users where email = 'clinician3@example.com')  as u3,
    (select id from auth.users where email = 'clinician4@example.com')  as u4,
    (select id from auth.users where email = 'clinician5@example.com')  as u5,
    (select id from auth.users where email = 'clinician6@example.com')  as u6
)
insert into public.team_members (team_id, user_id, practice_id)
select t.team_id, uid, t.practice_id
from t
cross join u
cross join unnest(array[u.u1, u.u2, u.u3, u.u4, u.u5, u.u6]) as uid
where uid is not null
on conflict (team_id, user_id) do nothing;
