CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.referees (
    referee_id UUID NOT NULL,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
);