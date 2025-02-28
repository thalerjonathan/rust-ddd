CREATE TABLE rustddd.domain_events_outbox (
    id UUID NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE rustddd.domain_events_inbox (
    id UUID NOT NULL,
    payload JSONB NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TYPE rustddd.domain_event_type AS ENUM ('Inbox', 'Outbox');

CREATE TABLE rustddd.domain_events (
    id UUID NOT NULL,
    event_type rustddd.domain_event_type NOT NULL,
    payload JSONB NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE OR REPLACE FUNCTION domain_event_notification_trigger() RETURNS TRIGGER as $domain_event_notification_trigger$
  BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('domain_event_inserted', '{"event_id": "' || NEW.id || '", "event_type": "' || NEW.event_type || '", "payload": ' || NEW.payload || ', "created_at": "' || NEW.created_at || '"}');
        RETURN NEW;
    END IF;
END;
$domain_event_notification_trigger$ LANGUAGE plpgsql;


CREATE TRIGGER domain_events_trigger
AFTER INSERT ON rustddd.domain_events FOR EACH ROW
EXECUTE PROCEDURE domain_event_notification_trigger();

INSERT INTO rustddd.domain_events (id, event_type, payload, created_at) VALUES ('f633920b-38d1-49d5-b9bc-467e25af7946', 'Inbox', '{}', NOW());