CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.venues (
    venue_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    street VARCHAR NOT NULL,
    zip VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    telephone VARCHAR,
    email VARCHAR
);

INSERT INTO rustddd.venues (venue_id, name, street, zip, city, telephone, email) VALUES
('6ee926bc-3728-4cdb-8efb-98d350a07854'::UUID, 'Venue A', 'Street A', '12345', 'City A', '1234567890', 'venuea@example.com'),
('cf49df42-cf40-48fa-b2e7-d31b4c796ce1'::UUID, 'Venue B', 'Street B', '23456', 'City B', '2345678901', 'venueb@example.com'),
('54e9b343-be07-4e08-a0b7-c82778aa1604'::UUID, 'Venue C', 'Street C', '34567', 'City C', '3456789012', 'venuec@example.com');
