-- Add migration script here

create schema if not exists jobs;

create table if not exists jobs.events (
    -- Used to order events.
    sequence_num bigserial not null,
    -- ID for the entity that the events apply to.
    stream_id uuid not null,
    -- Used to prevent concurrent updates on the same stream.
    version int not null,
    -- Event type, example: identity/CREATED.
    event_type text not null,
    -- Event payload containing all the state changes.
    data jsonb not null,
    -- Event metadata that helps track the events through our system.
    cid uuid not null,
    -- When the event got inserted into the DB.
    inserted_at timestamptz not null default now(),
    -- We primarily want to query the events in sequence.
    primary key (sequence_num),
    -- We block an update if it is based on an old stream version.
    unique (stream_id, version)
);

-- Helps us to query for all the events for a specific stream.
CREATE INDEX jobs_idx_event_stream_id ON jobs.events (stream_id);
