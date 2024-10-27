# Day 20: Adding Domain Events using Kafka

The plan for today was to add domain events using Kafka. 

The first use case is to implement the `RefereeClubChanged` domain event, which is triggered when the referee club is changed, so that services can react to this change (e.g. invalidate caches). The plan was to find suitable abstractions for the domain event consumer to avoid too much code duplication and boilerplate. 

## Results

Adding domain events was pretty straightforward, as expected, which I attribute to my previous knowledge of Kafka and/in Rust. I abstracted out common functionality into the shared library, and extended the domain events handler trait and implementation. This allowed me to quickly add publishing and consuming domain events to all services. So far the domain event listeners only log the events, with the exception of the *Referees* service, which invalidates a cache entry when the referee club is changed.

An important thing that I incorporated were transactional Kafka producers. The reason for that is that the Domain events are emitted from the application layer which means that they might be consumed in the same service before the DB TX has committed, which might lead to issues when the async handler needs to access the corresponding entry in the DB. The solution is to use a transactional producer, which will only commit the message after the DB TX has been committed, as it is done outside of the application layer, in the ports layer AFTER the DB TX has been committed. Therefore we also get the benefit of atomicity for the domain events: if the DB TX is rolled back, the domain event will not be published because the error will be thrown and the message will be discarded. A downside however is the case when the DB TX is committed but the Kafka TX is not, in which case we might have an inconsistency between the DB and Kafka.

## Conclusions

Cursor was a huge help for this task, as it easily anticipated the code needed to add new domain events. I only had to add the producer in the application layer and Cursor suggested the rest, as well as reasonable domain events in the shared library, and extending the domain events handler trait and implementation.

