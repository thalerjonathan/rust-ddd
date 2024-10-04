# Day 5: First steps towards DDD

So far, the backend code in the handlers is very functional and does not follow DDD principles. In particular it has the following shortcomings:
- Mix data logic and domain logic, loading data from DB into DTOs and returning them directly to the API consumer.
- There is no clear concept of the domain model in the handlers - yes, from the name of the handler function it is clear what is happening, but there is no clear concept of the domain model in the handlers. The whole domain logic is essentially implemented directly via DB queries. Yes, ultimately every domain ultimately ends up in the DB, but we want to represent the domain more explicitly and let the DB just be the storage layer.
- The code is difficult to test in an isolated way, as the handlers are very tightly coupled to the DB - the most straightforward way here is to test the handlers via E2E tests.

The first step towards DDD is to separate the data access layer from the domain logic:
- Implement a *Referee* aggregate that encapsulates the domain logic of the referee. Note that the domain logic that concerns the Referee is at the moment only *change Club*, but this minimal case will serve as a great starting point to develop the basic concepts of DDD in Rust.
- Introduce Domain Services that contain the actual domain logic 
- Introduce repositories where data access logic is concentrated and where we load Referee aggregates and persist them after changes.

Before we start, however, we gonna implement a number of E2E tests for the handlers, to have a good starting point to refactor the code. Also, it will serve us when we refactor the code from monolith to microservices. 

## Implementing the E2E tests

I implemented the E2E tests directly in Rust, into the `main.rs`, in the backend project. Note that these E2E tests are end-to-end from the perspective of the API consumer - for true E2E tests that cover the whole application we would need to test via the UI, using JavaScript tools like Selenium or Playwright.

Implementing the E2E tests was extremely easy. Due to Cursors insane context awareness, they basically wrote themselves, the only thing I had to do was to write out the test function name and Cursor seemed to understand what I wanted to do and came up with the correct test code.

## Refactoring towards DDD

## Conclusion

I was particularly impressed how well Cursor can help with writing the E2E tests and the test cases.