CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TYPE rustddd.fixture_status AS ENUM ('scheduled', 'cancelled');

CREATE TABLE IF NOT EXISTS rustddd.fixtures (
    fixture_id UUID NOT NULL PRIMARY KEY,
    team_home_id UUID NOT NULL,
    team_away_id UUID NOT NULL,
    venue_id UUID NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    status rustddd.fixture_status NOT NULL,
    first_referee_id UUID,
    second_referee_id UUID
);

CREATE TYPE rustddd.domain_event_type AS ENUM ('Inbox', 'Outbox');

CREATE TABLE rustddd.domain_events (
    id UUID NOT NULL,
    event_type rustddd.domain_event_type NOT NULL,
    payload JSONB NOT NULL,
    instance UUID NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE OR REPLACE FUNCTION domain_event_notification_trigger() RETURNS TRIGGER as $domain_event_notification_trigger$
  BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('domain_event_inserted', '{"event_id": "' || NEW.id || '", "event_type": "' || NEW.event_type || '", "instance": "' || NEW.instance || '", "payload": ' || NEW.payload || ', "created_at": "' || to_char(NEW.created_at, 'YYYY-MM-DD"T"HH24:MI:SS.MS"Z"') || '"}');
        RETURN NEW;
    END IF;
END;
$domain_event_notification_trigger$ LANGUAGE plpgsql;

CREATE TRIGGER domain_events_trigger
AFTER INSERT ON rustddd.domain_events FOR EACH ROW
EXECUTE PROCEDURE domain_event_notification_trigger();

INSERT INTO rustddd.fixtures (fixture_id, team_home_id, team_away_id, venue_id, date, status, first_referee_id, second_referee_id) VALUES
('ba045e60-1ae2-4902-8293-02b04747a888'::UUID, 'def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, '9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, '6ee926bc-3728-4cdb-8efb-98d350a07854'::UUID, '2024-01-01 10:00:00', 'scheduled', '2ef28cf5-6471-4051-ae11-0f419aef3234', 'e1214a09-42e1-4194-9acc-d310172d001a'),
('0aacbbba-1646-4478-8594-2401f19ad08d'::UUID, 'bca10019-1a77-48c6-a605-77c9289255b1'::UUID, 'def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, 'cf49df42-cf40-48fa-b2e7-d31b4c796ce1'::UUID, '2024-01-02 11:00:00', 'scheduled', '3bda5555-d604-432e-829a-78c782cccc18', 'e1214a09-42e1-4194-9acc-d310172d001a'),
('45c7140e-3361-40e6-b54c-d0af3f9c0749'::UUID, '9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, 'bca10019-1a77-48c6-a605-77c9289255b1'::UUID, '54e9b343-be07-4e08-a0b7-c82778aa1604'::UUID, '2024-01-03 12:00:00', 'cancelled', NULL, NULL);
