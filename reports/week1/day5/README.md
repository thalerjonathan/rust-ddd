# Day 5: First steps towards DDD

So far, the backend code in the handlers is very functional and does not follow DDD principles. In particular it has the following shortcomings:
- There are essentially no layers: all is handled within the "presentation" (REST handlers) layer and data logic and domain logic are mixed, loading data from DB into DTOs and returning them directly to the API consumer.
- There is no clear concept of the domain model in the handlers - yes, from the name of the handler function it is clear what is happening, but there is no clear concept of the domain model in the handlers. The whole domain logic is essentially implemented directly via DB queries. Yes, ultimately every domain ultimately ends up in the DB, but we want to represent the domain explicitly in code and let the DB just be the persistence layer.
- The code is difficult to test in an isolated way, as the handlers are very tightly coupled to the DB - the most straightforward way here is to test the handlers via E2E tests.

The first step towards DDD is to separate the data access layer from the domain logic:
- Introduce an Application Layer/Service that contains the application logic, separated from the presentation and domain logic. If you are more familiar with the Hexagonal Architecture, this is the "ports and adapters" architecture, where the the ports (REST) and the adapters (DB) are all built around an application core that contains the domain logic.
- Implement a *Referee* aggregate that encapsulates the domain logic of the referee. Note that the domain logic that concerns the Referee is at the moment only *change Club*, but this minimal case will serve as a great starting point to develop the basic concepts of DDD in Rust.
- Introduce repositories where data access logic is concentrated and where we load Referee aggregates and persist them after changes.

Before we start, however, we gonna implement a number of E2E tests for the handlers, to have a way of detecting regressions when refactoring, so they act as a safety net. Also, it will serve us when we undertake more fundamental refactoring of the code from monolith to microservices.

## Implementing the E2E tests

I implemented the E2E tests directly in Rust, into the `main.rs` module of the backend project. Note that these E2E tests are end-to-end from the perspective of the API consumer. For true E2E tests that cover the whole application we would need to test via the UI, using JavaScript tools like Selenium or Playwright - I don't want to go there for this project, as I want to focus 100% on using Rust.

Implementing the E2E tests was extremely easy. Due to Cursors insane context awareness, they basically wrote themselves, the only thing I had to do was to write out the test function name and Cursor seemed to understand what I wanted to do and came up with the correct test code.

## Refactoring towards DDD

I followed the steps outlined above and basically went with Cursors suggestsions, which were almost always spot on and I only had to make a few adjustments here and there. Essentially this allowed me to refactor the backend code into a proper hexagonal architecture and tests for the application layer in ~1 hour.

## Conclusion

I was particularly impressed how well Cursor can help with writing the E2E tests and the test cases. Also, the support from Cursor in refactoring towards DDD and writing tests for the application layer was just INSANE.

The work so far concludes the first week of the challenge. I have to say that after only 4 days of working with Cursor I am fully sold on the benefits of using an LLM to support software development: it gives you much more room to think and essentially takes away the tedious work of writing code that is not that interesting, but necessary - again, Software Engineering is not about typing (producing code) but about thinking, and LLMs are great at helping with exactly this.

Next week I am hoping to fully implement the whole domain logic and all user stories - let's see how this goes, but I am pretty confident that with the help of Cursor I will be able to pull it off.