# Day 21: Making Domain Events Handling Robust and Slicing out the `Availabilities` service

There are currently 3 problems with the Domain Event handling

1. There is a potential for processing the same event multiple times:  Currently the event handlers mostly only log the event, and only in the event of the Club of a Referee changed we invalidate the corresponding cache entry, which is an idempotent operation. However, for other events this might not be the case, and in general it certainly isn't. Therefore we need to implement a deduplication mechanism.

2. We are using Kafka transactions to atomically commit all produced domain events. However, in the case of a DB failure, the transaction is rolled back, but we are not rolling back the open Kafka transactions. This needs to be handled in a proper way. 

3. Also there is an edge case that committing the DB Tx goes through but the Kafka TX committing fails for whatever reason - in this case we would lose the domain event. We need to handle this case as well.

## Solution

I decided to tackle only the 2nd problem, as this was the easiest one to implement. The way I did it was via some async closures that are passed to a `run_transactionally` function that handles the begin/commit and rollback in error cases. 

## Results

Besides, I also sliced out the `Availabilities` service from the monolith, and did some refactoring of the other services, such as passing the Kafka TX id via program arguments.

## Conclusions

This was a pretty straightforward day, and I am happy with the results. The code is more robust and the potential for data loss is greatly reduced. Also having done the `Availabilities` service there is only one service left to go, that is the `Assignments` service, which I will do tomorrow.