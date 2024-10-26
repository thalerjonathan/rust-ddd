# Day 20: Adding Domain Events using Kafka

The plan for today was to add domain events using Kafka. 

The first use case is to implement the `RefereeClubChanged` domain event, which is triggered when the referee club is changed, so that services can react to this change (e.g. invalidate caches).

TODO
- Add domain events to *Fixtures* service
- Add domain events to *Venues* service
- Add domain events to *Teams* service
