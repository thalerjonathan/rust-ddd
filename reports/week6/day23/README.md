# Day 23: Fixing bugs, getting UI working and running microservices in parallel.

The plan for Day 23 was to:

- Get `Fixture` E2E tests working.
- Get UI working (there are some runtime errors  when running `trunk serve`)
- Run 2 instances of each microservice in parallel.

## Results

Fixing things was pretty straightforward:

- `Fixture` E2E tests are working - the issue was that the Domain Event handler in Fixtures didn't run and also didnt commit the DB Tx, therefore resulting in inconsistent states in the DB.
- UI is working - the solution was to remove the `axum` dependency from the `restinterface` dependency, which was done by creating a new `shared` crate and moving the `AppError` stuff into it.

