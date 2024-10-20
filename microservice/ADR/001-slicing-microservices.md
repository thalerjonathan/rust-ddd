# 1 - Slicing the existing Monolith into Microservices

## Status

Accepted

## Context

The existing Monolith is not able to scale to the required load. For example there are vastly different read and write loads for different parts of the system, which cannot be addressed by a monolithic architecture.

## Decision

We will slice the Monolith into multiple Microservices, where we gonna slice along Aggregate boundaries. Therefore we will end up with the following Microservices:

- Referees
- Teams
- Venues
- Fixtures
- Availabilities
- Assignments

## Consequences

Refactoring into Microservices has the far-reaching consequence that each Microservice gets its own DB to which only the Microservice has exclusiver access. Due to the fact that Microservices need to access data from other Microservices, we will need to implement some form of API Gateway, some form of Caching for performance reasons, there needs to be some form of Domain Event broadcasting mechanism and we need to be able to commit Assignments across Microservices. All of these are addressed in separate ADRs:

- [002 - Separate DB per Microservice](002-separate-dbs-per-microservice.md)
- [003 - API Gateway](003-api-gateway.md)
- [004 - Caching for Performance](004-caching-for-performance.md)
- [005 - Domain Event Broadcasting](005-domainevent-broadcasting.md)
- [006 - Saga for Committing Assignments Across Microservices](006-saga-committing-assignments-microservices.md)

## Notes

The "scaling" argument is obviously not really relevant for this project, as it is never gonna hit a load where it would be a problem, but it is a good argument for microservices in general, and it is a good argument to motivate the slicing of the Monolith into Microservices.