# Day 16

The plan for today was to kick off the refactoring of the monolith into microservices.

I started by writing [Architecture Decision Records](../../../microservice/ADR) for the overalle approach, and started with slicing out the Referee service.

## Conclusions

Slicing out the Referee service was pretty straightforward, basically just copy pasting from the monolith. 

I also added the API Gateway in the form of an Nginx container, that routes requests to `/referees` at port 3000 to the Referee service at port 4000.
