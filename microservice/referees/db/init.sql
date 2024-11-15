CREATE SCHEMA IF NOT EXISTS rustddd;

CREATE TABLE IF NOT EXISTS rustddd.referees (
    referee_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    club VARCHAR NOT NULL
);

CREATE TYPE rustddd.domain_event_type AS ENUM ('Inbox', 'Outbox');

CREATE TABLE rustddd.domain_events (
    id UUID PRIMARY KEY NOT NULL,
    event_type rustddd.domain_event_type NOT NULL,
    payload JSONB NOT NULL,
    instance UUID NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE OR REPLACE FUNCTION domain_event_notification_trigger() RETURNS TRIGGER as $domain_event_notification_trigger$
  BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('domain_event_inserted', '{"event_id": "' || NEW.id || '", "event_type": "' || NEW.event_type || '", "instance": "' || NEW.instance || '", "payload": ' || NEW.payload || '", "created_at": "' || NEW.created_at || '"}');
        RETURN NEW;
    END IF;
END;
$domain_event_notification_trigger$ LANGUAGE plpgsql;

CREATE TRIGGER domain_events_trigger
AFTER INSERT ON rustddd.domain_events FOR EACH ROW
EXECUTE PROCEDURE domain_event_notification_trigger();

INSERT INTO rustddd.referees (referee_id, name, club) VALUES
('2ef28cf5-6471-4051-ae11-0f419aef3234'::UUID, 'John Doe', 'Club A'),
('e1214a09-42e1-4194-9acc-d310172d001a'::UUID, 'Jane Smith', 'Club B'),
('3bda5555-d604-432e-829a-78c782cccc18'::UUID, 'Jim Beam', 'Club C');
