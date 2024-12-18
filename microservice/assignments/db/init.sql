CREATE SCHEMA IF NOT EXISTS rustddd;
ALTER SYSTEM SET wal_level = logical;

CREATE TYPE rustddd.assignment_status AS ENUM ('committed', 'staged');
CREATE TYPE rustddd.assignment_referee_role AS ENUM ('first', 'second');

CREATE TABLE IF NOT EXISTS rustddd.assignments (
    status rustddd.assignment_status NOT NULL,
    referee_role rustddd.assignment_referee_role NOT NULL,
    fixture_id UUID NOT NULL,
    referee_id UUID NOT NULL,
    UNIQUE (fixture_id, referee_id)
);
ALTER TABLE rustddd.assignments REPLICA IDENTITY FULL;

CREATE TABLE rustddd.domain_events_outbox (
    id UUID NOT NULL,
    instance UUID NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);
ALTER TABLE rustddd.domain_events_outbox REPLICA IDENTITY FULL;

CREATE TABLE rustddd.domain_events_inbox (
    id UUID NOT NULL,
    instance UUID NOT NULL,
    payload JSONB NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);
ALTER TABLE rustddd.domain_events_inbox REPLICA IDENTITY FULL;


INSERT INTO rustddd.assignments (status, fixture_id, referee_id, referee_role) VALUES
('committed', 'ba045e60-1ae2-4902-8293-02b04747a888'::UUID, '2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID, 'first'),
('committed', 'ba045e60-1ae2-4902-8293-02b04747a888'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'second'),
('committed', '0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, '3bda5555-d604-432e-829a-78c782cccc18'::UUID, 'first'),
('committed', '0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'second');
