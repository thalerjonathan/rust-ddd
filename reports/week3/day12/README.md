# Day 12: Implementing Availabilities

The plan for today was to implement the *Availabilities* part, that is: given a selected Referee the system should display all fixtures and allow the referee to declare and withdraw their availability for a fixture.

The design I want to follow is to have a simple table that stores an entry for each availability for a given fixture: if there is an entry in the table, the referee is available for the fixture, if not, the referee is not available for the fixture. Therefore for the first time we implement a DELETE operation in this project. Also, there won't be an explicity *Availability* Aggregate because there is no need for it: we always refer to availabilities in the context of a single referee therefore we are left with a List of FixtureIds and pairs of RefereeId and FixtureId.
I am not going to merge the availabilities into the *Fixture* or *Referee* Aggregate because I want to keep them separated and free from the *Availability* concern, therefore implementing a separate *Availabilities* Aggregate and endpoints to fetch availabilities. This keeps things separated and compositional, and when refactoring/slicing the project into services, this will be beneficial.

Again, the plan was to follow a TDD approach, driven by E2E tests, that is:
1. extend DB schema.
2. add shared types and request/response functions.
3. write E2E tests.
4. add REST handlers.
5. implement DDD part.
6. implement Application services.
7. E2E tests should pass.
8. implement UI part in leptos.

## Results

I started by extending the DB schema - Cursor immediately suggested the correct table and columns and I went with it.

Adding shared types and request/response functions was straightforward, with Cursor providing perfect support.

## Conclusion