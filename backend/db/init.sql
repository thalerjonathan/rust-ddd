CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.referees (
    referee_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS rustddd.venues (
    venue_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    street VARCHAR NOT NULL,
    zip VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    telephone VARCHAR,
    email VARCHAR
);