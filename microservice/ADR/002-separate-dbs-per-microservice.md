# 2 - Separate DB per Microservice

## Status

Accepted

## Context

Microservices need exclusive access to DB to enable scaling each Microservice independently.

## Decision

Each Microservice needs to have its own DB to which only the Microservice has exclusive access.

## Consequences

If Microservices need to access data from other Microservices, we will need to implement some form of API Gateway, see [003 - API Gateway](006-api-gateway.md)
