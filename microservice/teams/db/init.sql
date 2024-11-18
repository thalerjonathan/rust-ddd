CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.teams (
    team_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
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

INSERT INTO rustddd.teams (team_id, name, club) VALUES
('def7f2ca-58a1-44ed-8f2b-78386c9746cf'::UUID, 'Team A', 'Club A'),
('9b93e265-deb3-4139-a9b8-e261d7985a05'::UUID, 'Team B', 'Club B'),
('bca10019-1a77-48c6-a605-77c9289255b1'::UUID, 'Team C', 'Club C');
