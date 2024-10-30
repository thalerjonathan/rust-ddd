# Day 22: Slicing Assignments and implementing a simple Saga

For Day 22 the plan was to slice the assignments microservice and implement a simple Saga to manage the assignment process.

The reason why we need a Saga is that when committing an Assignment as a reaction the corresponding `Fixture` Referees needs to be updated. The way I'm going to implement this for now is via Domain Events, constituting a VERY simple Saga.

## Results

The Saga is non-robust and does not handle errors or retries, which means it can leave the system in an inconsistent state in the event of failure. For example if the `Fixture` update fails, the `Assignment` is committed, leaving the system in an inconsistent state.

## Conclusions

The following things are left now:

- Get `Fixture` E2E tests working.
- Get UI working (there are some runtime errors  when running `trunk serve`)
- Properly implement the Saga pattern.
- Idempotent Domain Event handling (see [Day 21](../day21/README.md)).
- Reconcile the DB and Kafka TX contexts - maybe possible via a Saga (see [Day 21](../day21/README.md)).