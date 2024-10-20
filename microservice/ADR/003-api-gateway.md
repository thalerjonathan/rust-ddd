# 3 - API Gateway

## Status

Accepted

## Context

Microservices need to access data from other Microservices. This needs to happen in a transparent way for the calling Microservice, ideally in the same way as the frontend does.

## Decision

Implement an API Gateway in front of all Microservices, in form of a reverse proxy, that exposes a single API for the frontend to call, while internally routing requests to the appropriate Microservice.

## Consequences

Need to find a suitable reverse proxy that supports the required functionality.

## Notes

Nginx is a good candidate, but there are other options like Traefik or HAProxy.