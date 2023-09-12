create table projects
(
    id     int8 primary key,
    active boolean default true
);

create table visitors
(
    id       text primary key,
    project  int8 not null references projects (id),
    region   text,
    timezone text,
    language text,
    browser  text,
    platform text,
    width    int,
    height   int
);
create index on visitors (project);

create table pages
(
    id      text primary key,
    project int8 not null references projects (id),
    host    text,
    path    text
);
create index on pages (project);

create table utm_params
(
    id       text primary key,
    project  int8 not null references projects (id),
    campaign text,
    content  text,
    medium   text,
    source   text,
    term     text
);
create index on utm_params (project);

create table referrers
(
    id      text primary key,
    project int8 not null references projects (id),
    host    text,
    path    text
);
create index on referrers (project);

create table visits
(
    time      timestamptz not null default now(),
    project   int8        not null,
    session   text,
    visitor   text,
    page      text,
    utm_param text,
    referrer  text,
    duration  int,
    distance  float8
);

create table events
(
    time    timestamptz not null default now(),
    project int8        not null,
    session text,
    visitor text,
    page    text,
    name    text,
    data    jsonb
);
