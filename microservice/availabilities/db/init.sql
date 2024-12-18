CREATE SCHEMA IF NOT EXISTS rustddd;
ALTER SYSTEM SET wal_level = logical;

CREATE TABLE IF NOT EXISTS rustddd.availabilities (
    fixture_id UUID NOT NULL,
    referee_id UUID NOT NULL,
    UNIQUE (fixture_id, referee_id)
);
ALTER TABLE rustddd.availabilities REPLICA IDENTITY FULL;

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

INSERT INTO rustddd.availabilities (fixture_id, referee_id) VALUES
('ba045e60-1ae2-4902-8293-02b04747a888'::UUID, '2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID),
('ba045e60-1ae2-4902-8293-02b04747a888'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID),
('0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, '3bda5555-d604-432e-829a-78c782cccc18'::UUID),
('0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, 'e1214a09-42e1-4194-9acc-d310172d001a'::UUID),
('45c7140e-3361-40e6-b54c-d0af3f9c0749'::UUID, '2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID),
('45c7140e-3361-40e6-b54c-d0af3f9c0749'::UUID, '3bda5555-d604-432e-829a-78c782cccc18'::UUID);
