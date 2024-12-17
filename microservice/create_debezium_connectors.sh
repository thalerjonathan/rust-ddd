ASSGINMENTS_CONNECTOR_JSON="$(cat debezium/connector-config-assignments.json)"
echo "'$ASSGINMENTS_CONNECTOR_JSON'"

curl -X POST \
    'http://localhost:8083/connectors' \
    --header 'Accept: */*' \
    --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
    --header 'Content-Type: application/json' \
    --data-raw "$ASSGINMENTS_CONNECTOR_JSON"