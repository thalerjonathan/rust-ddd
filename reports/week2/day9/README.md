# Day 9

Today the plan is to finish the *Fixture* part.

The following features are still missing:
- Change Date of a *Fixture*.
- Change Venue of a *Fixture*.
- Cancelling of a *Fixture*.

Also, what is missing are checking the constraints when creating a new Fixture and also implementing the cancel feature.

The following constraints have to be respected:
- Both teams have to be different - can be checked as well in the frontend.
- No other *Fixture* is scheduled at the same venue and time.
- No other *Fixture* is scheduled at the same time for either of the two teams.
- The date of the *Fixture* is in the future.

An important TODO that I want to address is also changing the *Repositories* from receiving a connection pool to receiving a concrete connection, that would allow to enforce transactional guarantees that are generally required for operations on *Aggregates* in DDD. Yes, currently the repositories do not have any transactional logic, but the way they are implemented now is that they are fetching a new connection on every function call, which means that the transactional guarantee is lost.

## Results

The refactorings and the implementation of the *Fixture* features were done quite fast due to the help of Cursor. 

## Conclusions

So far I am quite happy with the DDD approach and the separation of concerns between the aggregates and the repositories. Rust helps a lot with its support for specifying mutability/immutability, which allows to enforce certain constraints already at compile time. For example the *Fixture* aggregate contains other entities but they are returned as references from an immutable *Fixture* aggregate - which makes sure that nothing can change the state of the *Fixture* or its related entities. The one thing that is not true to DDD atm is how I implement the *save* functionality in the the concrete Repositories: in case an *Entity*/*Aggregate* can change in certain ways, I have implemented an *upsert* functionality which updates only the fields that are allowed to change according to the Domain Logic/Model. The benefit is that I can use the *save* functionality in the *Repository* for both *Entity* creation and *Entity* update, but the downside is that Domain Logic is slipping into the actual SQL code - something that hardcore DDD purists would probably not like. On the other hand, a super-clean solution would require the implementation of a unit-of-work pattern, tracking changes and only applying them in the *save* functionality - something that I think would be much more difficult to implement in Rust and that I deem currently out of scope.