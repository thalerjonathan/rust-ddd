# NOTE: we create the debezium connector topics manually otherwise the consumers can not subscribe if they dont exist yet (and Debezium creates them only when the first event is written - so its a chicken-egg problem)
sh debezium/create_domain_events_topics.sh

ASSGINMENTS_CONNECTOR_JSON="$(cat debezium/connector-config-assignments.json)"
echo "ASSGINMENTS_CONNECTOR_JSON: $ASSGINMENTS_CONNECTOR_JSON"
curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$ASSGINMENTS_CONNECTOR_JSON"

AVAILABILITIES_CONNECTOR_JSON="$(cat debezium/connector-config-availabilities.json)"
echo "AVAILABILITIES_CONNECTOR_JSON: $AVAILABILITIES_CONNECTOR_JSON"
curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$AVAILABILITIES_CONNECTOR_JSON"

FIXTURES_CONNECTOR_JSON="$(cat debezium/connector-config-fixtures.json)"
echo "FIXTURES_CONNECTOR_JSON: $FIXTURES_CONNECTOR_JSON"
curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$FIXTURES_CONNECTOR_JSON"

REFEREES_CONNECTOR_JSON="$(cat debezium/connector-config-referees.json)"
echo "REFEREES_CONNECTOR_JSON: $REFEREES_CONNECTOR_JSON"
curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$REFEREES_CONNECTOR_JSON"

TOPICS_CONNECTOR_JSON="$(cat debezium/connector-config-topics.json)"
echo "TOPICS_CONNECTOR_JSON: $TOPICS_CONNECTOR_JSON"
curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$TOPICS_CONNECTOR_JSON"

VENUES_CONNECTOR_JSON="$(cat debezium/connector-config-venues.json)"
echo "VENUES_CONNECTOR_JSON: $VENUES_CONNECTOR_JSON"
curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$VENUES_CONNECTOR_JSON"
