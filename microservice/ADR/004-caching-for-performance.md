# 4 - Caching for Performance

## Status

Accepted

## Context

Microservices need to be highly performant to provide a good user experience. However due to the fact that Microservices have now separate DBs, it is not possible to JOIN over tables to fetch data from multiple tables which means that Aggregate instances need to be resolved via REST calls to other Microservices. For example when fetching Fixtures the REST endpoint returns a DTO which also contains full info on assigned Referees (if there are any), Teams and Venue, which all are going to reside in their own Microservice, therefore we cannot use the JOIN approach when fetching Fixtures but we need to fetch Referees, Teams and Venues separately by their id and resolve them in the backend service. For each row this would require up to 1+2+2 (Venue+Teams+Referees) additional REST queries to other services for full resolution.

## Decision

Implement a caching mechanism to reduce the REST roundtrips and keep performance high.

## Consequences

If the underlying data of a cached Aggregate changes, either the cache needs to be invalidated or the cache-entry needs to be updated. This should be possible by leveraging Domain Events, see [005 - Domain Event Broadcasting](005-domainevent-broadcasting.md)

## Notes

Redis is a good candidate, but there are other options like Memcached or Varnish.