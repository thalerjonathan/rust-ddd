CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.referees (
    referee_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
);

INSERT INTO rustddd.referees (referee_id, name, club) VALUES
('2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID, 'John Doe', 'Club A'),
('e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'Jane Smith', 'Club B'),
('3bda5555-d604-432e-829a-78c782cccc18'::UUID, 'Jim Beam', 'Club C');
