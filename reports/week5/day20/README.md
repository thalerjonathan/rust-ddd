# Day 20: Adding Domain Events using Kafka

The plan for today was to add domain events using Kafka. 

The first use case is to implement the `RefereeClubChanged` domain event, which is triggered when the referee club is changed, so that services can react to this change (e.g. invalidate caches).

