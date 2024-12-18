# Refactoring towards a more robust Outbox Pattern with Debezium

Note: this work was done on 17th and 18th December.

So far to deal with the issues of reliable message delivery (see week7 for a detailed discussion of the problem) I have implemented the [outbox pattern](https://microservices.io/patterns/data/transactional-outbox.html). In a nutshell what happens is this:
1. Domain Event is written into an Outbox table.
2. Using Postgres notification mechanism a separate "processor" thread is notified that emits the event to the Kafka topic (and retries in case of failure).
3. Event is processed by the receiving service, also by writing into an Inbox table, to implement deduplication.

The Postgres notification mechanism with the async "processor" thread I used is one way to flush the events to Kafka. However, there also exists Debezium, that provides DB connectors that do just that, by tailing the Transaction Log. Interestingly there is a [very well-written blog on the debezium](https://debezium.io/blog/2019/02/19/reliable-microservices-data-exchange-with-the-outbox-pattern/) site that explains in technical depth the outbox pattern and how to implement it with Debezium. 

The idea was to refactor the current "async processor" with Postgres notification approach to a Debezium approach, given it has a few [benefits over a polling/notification approach](https://debezium.io/blog/2018/07/19/advantages-of-log-based-change-data-capture/).
- The Debezium connector might also result in duplicates, but we can already deal with them via our Inbox approach.
- The Debezium example uses different Kafka topics for different Aggregates and also assigns Aggregate ids as keys to the Kafka messages. Given that we broadcast all Domain Events via a single Kafka topic and do not require that same Aggregates are processed on the same partition to respect ordering, this is not relevant in this project: we don't have semantics that need strict ordering in our Aggregates and we have only a single Kafka topic. However, if there are different topics and we we need to retain ordering between Aggregate Events, then this approach needs to be used as well.

The steps for refactoring where as followed:
1. Set up Debezium Connectors for each Postgres DB for each Microservice.
2. Split into an actual Outbox and Inbox table - currently it is implemented via a single table.
3. Throw out the async processor thread.
4. Refactor the inbound processor to the new JSON format.

## Results

Setting up Debezium and the Connectors was non-trivial. Important details here:
- You need to manually create topics for the CONFIG_STORAGE_TOPIC and OFFSET_STORAGE_TOPIC where both need `cleanup.policy=compact`.
- You need to configure the Postgres DB with `ALTER SYSTEM SET wal_level = logical;` so that Debezium can track the logical changes. 
- You need to add `ALTER TABLE table_name REPLICA IDENTITY FULL` to each table due to the `ALTER SYSTEM SET wal_level = logical` change, otherwise e.g. `DELETE` won't work.

The rest was rather straightforward, thanks to Rusts strong typesystem, the refactoring wasnt too difficult. A major change however was that each service has its own domain events topic: the format of the messages are in all cases the same, however it means that the consumeres now need to subscribe to multiple topics, instead of just one. This is however also a big benefit, as it allows to subscribe only to those topics that are really relevant for the given service, for example the Venues service does not need to register for any Domain Events emitted from the Assignments service because it just doesn't do anything with them.