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

When implementing the E2E tests, Cursor generated invalid code, by generating 2 `Path` parameters instead of one tuple of 2 `Path` parameters. I didn't realise this in the beginning, and didn't know axum well enough to realise it straight away. Only after I used ThunderClient to debug the problem, I realised the mistake, because I received a proper error message. 

## Conclusion

Overall today was pretty straightforward, and I was able to fully implement the backend part. 

The case with 2 `Path` parameters cost me around 1 hour to figure out, because I was debugging into different directions. This is a case where Cursor actually made the coding experience worse, because it led me to believe that the code it generated was correct. It clearly showed again that you need to have a (very) good understanding of the framework and the concepts at play, otherwise you end up spending a lot of time figuring out why the generated code is not working.

Due to the time spent on finding the bug, I didn't manage to implement the full UI part today, and am going to leave that for tomorrow / the next days.