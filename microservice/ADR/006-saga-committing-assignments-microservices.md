# 6 - Saga for Committing Assignments Across Microservices

## Status

Accepted

## Context

When committing Assignments across Microservices, the Assignment needs to be updated in the Assignments service as well as Referees written to the corresponding Fixture in the Fixtures service. This needs to happen in a transactional way, otherwise we might end up with inconsistent data.

## Decision

Employ a Saga to commit Assignments across Microservices.

## Consequences

Substantial increase of complexity, and potentially additional 

