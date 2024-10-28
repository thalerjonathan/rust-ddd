# Day 21: Making Domain Events Handling Robust

There are currently 3 problems with the Domain Event handling

1. There is a potential for processing the same event multiple times:  Currently the event handlers mostly only log the event, and only in the event of the Club of a Referee changed we invalidate the corresponding cache entry, which is an idempotent operation. However, for other events this might not be the case, and in general it certainly isn't. Therefore we need to implement a deduplication mechanism.

2. We are using Kafka transactions to atomically commit all produced domain events. However, in the case of a DB failure, the transaction is rolled back, but we are not rolling back the open Kafka transactions. This needs to be handled in a proper way. 

3. Also there is an edge case that committing the DB Tx goes through but the Kafka TX committing fails for whatever reason - in this case we would lose the domain event. We need to handle this case as well.