CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.teams (
    team_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
);

INSERT INTO rustddd.teams (team_id, name, club) VALUES
('def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, 'Team A', 'Club A'),
('9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, 'Team B', 'Club B'),
('bca10019-1a77-48c6-a605-77c9289255b1'::UUID, 'Team C', 'Club C');