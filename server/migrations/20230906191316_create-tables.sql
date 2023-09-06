create table projects
(
    id     int8 primary key,
    active boolean default true
);

create table visitors
(
    id       int8 primary key,
    region   text,
    timezone text,
    language text,
    browser  text,
    platform text,
    width    int,
    height   int
);

create table pages
(
    id   int8 primary key generated always as identity,
    host text,
    path text
);

create table utm_params
(
    id       int8 primary key generated always as identity,
    campaign text,
    content  text,
    medium   text,
    source   text,
    term     text
);

create table referrers
(
    id   int8 primary key generated always as identity,
    host text,
    path text
);

create table visits
(
    instant   timestamptz not null default now(),
    visitor   int8,
    page      int8,
    utm_param int8,
    referrer  int8
);

create table exit_states
(
    instant  timestamptz not null default now(),
    visitor  int8,
    page     int8,
    time     int,
    distance double precision
);

create table events
(
    instant timestamptz not null default now(),
    visitor int8,
    page    int8,
    name    text,
    data    jsonb
);
