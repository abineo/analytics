select create_hypertable(
               'visits',
               'time',
               create_default_indexes => false
           );
create index on visits (project, time);
alter table visits
    set (
        timescaledb.compress,
        timescaledb.compress_segmentby = 'project',
        timescaledb.compress_orderby = 'project, time DESC'
        );
select add_compression_policy('visits', INTERVAL '1 month');

select create_hypertable(
               'events',
               'time',
               create_default_indexes => false
           );
create index on events (project, time);
alter table events
    set (
        timescaledb.compress,
        timescaledb.compress_segmentby = 'project',
        timescaledb.compress_orderby = 'project, time DESC'
        );
select add_compression_policy('events', INTERVAL '1 month');
