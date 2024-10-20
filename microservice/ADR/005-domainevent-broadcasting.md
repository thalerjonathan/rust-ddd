# 5 - Domain Event Broadcasting

## Status

Accepted

## Context

Microservices need to be able to react to changes in other Microservices, for example to update data in their DB or to invalidate cache entries.

## Decision

Use a message broker for broadcasting Domain Events between Microservices.

## Consequences

We need to implement some form of idempotency handling due to different transactional contexts between Message Broker and Cache/DB updates.

## Notes

Kafka is a good candidate, but there are other options like NATS.io or RabbitMQ.