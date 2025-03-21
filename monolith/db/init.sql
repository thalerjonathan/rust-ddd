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

CREATE TYPE rustddd.fixture_status AS ENUM ('scheduled', 'cancelled');

CREATE TABLE IF NOT EXISTS rustddd.fixtures (
    fixture_id UUID NOT NULL PRIMARY KEY,
    team_home_id UUID NOT NULL,
    team_away_id UUID NOT NULL,
    venue_id UUID NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    status rustddd.fixture_status NOT NULL,
    first_referee_id UUID,
    second_referee_id UUID,
    CONSTRAINT fk_team_home FOREIGN KEY (team_home_id) REFERENCES rustddd.teams(team_id),
    CONSTRAINT fk_team_away FOREIGN KEY (team_away_id) REFERENCES rustddd.teams(team_id),
    CONSTRAINT fk_venue FOREIGN KEY (venue_id) REFERENCES rustddd.venues(venue_id),
    CONSTRAINT fk_first_referee FOREIGN KEY (first_referee_id) REFERENCES rustddd.referees(referee_id),
    CONSTRAINT fk_second_referee FOREIGN KEY (second_referee_id) REFERENCES rustddd.referees(referee_id)
);

CREATE TABLE IF NOT EXISTS rustddd.availabilities (
    fixture_id UUID NOT NULL,
    referee_id UUID NOT NULL,
    CONSTRAINT fk_fixture FOREIGN KEY (fixture_id) REFERENCES rustddd.fixtures(fixture_id),
    CONSTRAINT fk_referee FOREIGN KEY (referee_id) REFERENCES rustddd.referees(referee_id),
    UNIQUE (fixture_id, referee_id)
);

CREATE TYPE rustddd.assignment_status AS ENUM ('committed', 'staged');
CREATE TYPE rustddd.assignment_referee_role AS ENUM ('first', 'second');

CREATE TABLE IF NOT EXISTS rustddd.assignments (
    status rustddd.assignment_status NOT NULL,
    fixture_id UUID NOT NULL,
    referee_id UUID NOT NULL,
    referee_role rustddd.assignment_referee_role NOT NULL,
    CONSTRAINT fk_fixture FOREIGN KEY (fixture_id) REFERENCES rustddd.fixtures(fixture_id),
    CONSTRAINT fk_referee FOREIGN KEY (referee_id) REFERENCES rustddd.referees(referee_id),
    UNIQUE (fixture_id, referee_id)
);

INSERT INTO rustddd.referees (referee_id, name, club) VALUES
('2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID, 'John Doe', 'Club A'),
('e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'Jane Smith', 'Club B'),
('3bda5555-d604-432e-829a-78c782cccc18'::UUID, 'Jim Beam', 'Club C');

INSERT INTO rustddd.venues (venue_id, name, street, zip, city, telephone, email) VALUES
('6ee926bc-3728-4cdb-8efb-98d350a07854'::UUID, 'Venue A', 'Street A', '12345', 'City A', '1234567890', 'venuea@example.com'),
('cf49df42-cf40-48fa-b2e7-d31b4c796ce1'::UUID, 'Venue B', 'Street B', '23456', 'City B', '2345678901', 'venueb@example.com'),
('54e9b343-be07-4e08-a0b7-c82778aa1604'::UUID, 'Venue C', 'Street C', '34567', 'City C', '3456789012', 'venuec@example.com');

INSERT INTO rustddd.teams (team_id, name, club) VALUES
('def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, 'Team A', 'Club A'),
('9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, 'Team B', 'Club B'),
('bca10019-1a77-48c6-a605-77c9289255b1'::UUID, 'Team C', 'Club C');

INSERT INTO rustddd.fixtures (fixture_id, team_home_id, team_away_id, venue_id, date, status, first_referee_id, second_referee_id) VALUES
('ba045e60-1ae2-4902-8293-02b04747a888'::UUID, 'def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, '9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, '6ee926bc-3728-4cdb-8efb-98d350a07854'::UUID, '2024-01-01 10:00:00', 'scheduled', '2ef28cf5-6471-4051-ae11-0f419aef3234', 'e1214a09-42e1-4194-9acc-d310172d001a'),
('0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, 'bca10019-1a77-48c6-a605-77c9289255b1'::UUID, 'def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, 'cf49df42-cf40-48fa-b2e7-d31b4c796ce1'::UUID, '2024-01-02 11:00:00', 'scheduled', '3bda5555-d604-432e-829a-78c782cccc18', 'e1214a09-42e1-4194-9acc-d310172d001a'),
('45c7140e-3361-40e6-b54c-d0af3f9c0749'::UUID, '9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, 'bca10019-1a77-48c6-a605-77c9289255b1'::UUID, '54e9b343-be07-4e08-a0b7-c82778aa1604'::UUID, '2024-01-03 12:00:00', 'cancelled', NULL, NULL);

INSERT INTO rustddd.availabilities (fixture_id, referee_id) VALUES
('ba045e60-1ae2-4902-8293-02b04747a888'::UUID, '2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID),
('ba045e60-1ae2-4902-8293-02b04747a888'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID),
('0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, '3bda5555-d604-432e-829a-78c782cccc18'::UUID),
('0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID),
('45c7140e-3361-40e6-b54c-d0af3f9c0749'::UUID, '2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID),
('45c7140e-3361-40e6-b54c-d0af3f9c0749'::UUID, '3bda5555-d604-432e-829a-78c782cccc18'::UUID);

INSERT INTO rustddd.assignments (status, fixture_id, referee_id, referee_role) VALUES
('committed', 'ba045e60-1ae2-4902-8293-02b04747a888'::UUID, '2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID, 'first'),
('committed', 'ba045e60-1ae2-4902-8293-02b04747a888'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'second'),
('committed', '0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, '3bda5555-d604-432e-829a-78c782cccc18'::UUID, 'first'),
('committed', '0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'second');