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

CREATE TABLE IF NOT EXISTS rustddd.teams (
    team_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
);

INSERT INTO rustddd.referees (referee_id, name, club) VALUES
(gen_random_uuid(), 'John Doe', 'Club A'),
(gen_random_uuid(), 'Jane Doe', 'Club B'),
(gen_random_uuid(), 'Jim Beam', 'Club C');

INSERT INTO rustddd.venues (venue_id, name, street, zip, city, telephone, email) VALUES
(gen_random_uuid(), 'Venue A', 'Street A', '12345', 'City A', '1234567890', 'venuea@example.com'),
(gen_random_uuid(), 'Venue B', 'Street B', '23456', 'City B', '2345678901', 'venueb@example.com'),
(gen_random_uuid(), 'Venue C', 'Street C', '34567', 'City C', '3456789012', 'venuec@example.com');

INSERT INTO rustddd.teams (team_id, name, club) VALUES
(gen_random_uuid(), 'Team A', 'Club A'),
(gen_random_uuid(), 'Team B', 'Club B'),
(gen_random_uuid(), 'Team C', 'Club C');
