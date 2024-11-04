# Exploring DDD and Microservices with the help of LLMs in Rust 

This repo explores the use of Rust for implementing DDD and Microservices with the help of LLMs on top of a simple web UI in Rust with WASM. 

The Domain which I use as example is a simple Volleyball referee management tool, as I am very familiar with this domain. I have been Volleyball referee for many years in the past, as well as implemented a (much more complex) tool in Java (also following DDD) for the Vorarlberg Volleyball Association, which I used in production during my tenure as Volleyball referee manager of Vorarlberg. The existing tool in Java essentially implements the same Domain but is technically different (Monolith, integration with an existing system).

UI wise, I am using Leptos, due to it being rather close to React which I have some experience in.

Architectural wise, I start simple with a monolith architecture and then transition it into  Microservices. 

Note that in the implementation of this project I have used LLMs in my coding for the first time, where I focused on the use of Cursor and ChatGPT.

## Day 1-15: Reflections on the DDD Monolith

I managed to finish a monolithic DDD implementation within 15 working days (over 3 weeks, not working on weekends). Due to interview processes/preparationsw and other obligations I could contributed less time than expected. (Unfortunately) I didn't track the hours I spent on each day, but on average it was 3-4 hours per day, so totalling at ~50 hours after day 15. In my experience doing deep docus work of 4-5 hours a day is pretty much the limit - even in a full remote job you won't get much more than that, because either you are distracted by other things or you get tired, as deep focus work of more than 5 hours is very draining.

As a consequence of the limited time, I have not implemented every detail of all user stories, as the focus is more on technology experimentation - for example notification of referees is missing, as well as locking/unlocking availabilities. Also, the UI is super simple but here is no proper error handling of the REST results on UI side and there is obvious room for carving out parent/child relations using components, especially in the Assignments component. Also, the error handling in the backend is very generic, and essentially only returns errors in the form of Strings. I also didn't implement *validation* of staged assigments, which is very complex domain logic. I expect that addressing these limitations would need another 30 hours, totalling in total work of ~2 weeks when seen from a "job perspective".

### DDD Implementation
In a nutshell my DDD implementation boils down to the following key points:

- For each Repository trait there exists an actual *Postgres* implementation that translates the methods to Postgres queries.
- For loading rows from the DB, I have created a separate DB-struct in each Repository implementation that reflects the colums returned by the query. From this DB-struct the Aggregate is then instantiated and returned.
- To go between DB-structs, Aggregates and DTOs there exist a large number of `From` instance implementations, which make the transformations very convenient using the `into` method.
- Aggregates are completely DB- and technology agnostic and hold only fields and asociated functions that are either immutable getters or mutable domain logic. The transformation from DB-structs is private to the Repository implementation and is therefore not leaked outside.
- The fact that Rust allows to define mutable/immutable properties on Aggregate (associated) functions, allows to enforce domain logic semantics at compile time. This way getters/query functions can be declared immutable and mutating domain logic as mutable. 
- The transactional boundaries are currently handled in the REST layer, which might not appeal to DDD purists, which claim they should be put into the application service layer. However given the difficulties I had with abstracting the Transaction/Connection object out, I am happy with the current solution, which allows to conveniently write tests for the application services, using mocks for the aggregates. A direction for future work might be to come up with  "annotations" for the application services that demarcate transactional boundaries.
- The way that transactional boundaries work is that the REST layer begins a TX using the connection pool and passes this in an abstract form to the application layer which then passes it on to the Aggregates.
- I settled on the need to explicit call `save` on an Aggregate in case an Aggregate has changed or is new. I implemented `save` as an upsert which allows to use the same method in case of an insert or update, which simplifies things. However despite the convenience of the upsert, there is huge room for future improvement such as caching, lazy loading and unit-of-work, which all exists already for decades in Hibernate.
- Loading Fixtures currently JOINS over 5 tables (1 Venue, 2 Teams, 2 Referees). Given that relational DBs like Postgres are superb at dealing with JOINS I don't see this as a big problem for now. However when the tables grow and performance goes down, functionality like `get_all` should be disencouraged or removed alltogether and limited to querying over certain time window of e.g. up to 1 month or implement some form of paging.  

### Takeaways
Overall I am quite happy with where I got in this limited, time, and I would go as far as using the backend concepts in production, if I had to write a DDD application in Rust.

When it comes to implementing DDD in Rust, I found that - unsurprisingly - it all boils down to the persistence layer. To be more specific: the question right from the beginning was how to best persist Aggregates and how to deal with transactional boundaries. When you come from the Java (EE) world as I do, then you are used to frameworks like Spring and Hibernate, which are amazing pieces of technology. They have matured over ~20 years with uncountable of working hours contributed to them, as well as uncountable industry lessons-learned went into their implementation. Nothing like that exists in Rust (yet) - yes there are "ORMs" like SeaORM or Diesel but I didn't want to touch them as I have used Diesel in the past and was not very happy with it. Therefore I decided to go quite low-level using SQLx and doing everything "by hand". Despite having full control, SQLx still gave me some headache (for example when dealing with nullables or enums, as it seems that SQLx has yet some time to full maturiy) but to be honest this was expected: DB abstractions are ALWAYS causing headaches, whether it is Hibernate, Diesel or SQLx - its just a very non-trivial topic.

Using Rust WASM via Leptos was a very nice experience, especially if you are familiar with a framework like React: writing everything in Rust from backend to frontend is extremely convenient and fast, as you can use same types in form of a shared library, and don't need to resort to things like OpenAPI definitions/generators. However I have to admit that I am not experienced enought to properly judge whether Rust WASM/Leptos is good enough to implement a full, complex production-ready Web frontend.

Cursor/the use of LLMs was a tremendeous help and surpassed all my expectations. Although I started using it by asking it to generate code up-front, I eventually settled on using it only to for its suggestions, giving it some contextual information either by starting to type something or opening files. However what became clear very quickly was that it does not do the thinking for you, you need to know what you want to do and you need to lead the way. This didn't come as a surprise but essentially confirmed my expectations of Cursor being an assistant that given your thinking will produce the right "typing" for you, therefore taking away the annoying task of typing (Software Engineering is not about producing/typing code but about thinking) - if you don't have multiple years of working experience in a certain programming language where you haven't yet memorised the complete syntax of all edge cases, this is very handy. However, the question remains: *How would you learn the syntax if you never actually type it?* This question I think points to a deeper *issue* with AI and the use of LLMs: how can you truly learn something from the LLM if it produces the thing for you? For junior devs and students this is a VERY big problem. The jury is still out on this one, but we can expect that it is going to change the way we learn how to write and how to actually write software forever. 

### Microservices Outlook

In the next phase I am going to slice the monolith into a Microservice architecture. Probably I am going to keep the monolith implementation and come up with the Microservices alongside of it within the same (multi-project) repo.

When transitioning to Microservices we gonna see the following consequences:

- Using Redis to cache Aggregate instances across Microservices, which is necessary to arrive at acceptable performance. The problem is this: when fetching Fixtures the REST endpoint returns a DTO which also contains full info on assigned Referees (if there are any), Teams and Venue, which all are going to reside in their own Microservice, therefore we cannot use the JOIN approach when fetching Fixtures but we need to fetch Referees, Teams and Venues separately by their id and resolve them in the backend service. For each row this would require up to 1+2+2 (Venue+Teams+Referees) additional REST queries to other services for full resolution - given that Venues, Teams and Referees change very rarely (or in most circumstances not at all), they can be cached via Redis. If this is not done, I expect a dramatic negative performance impact. Given that Venues, Teams and Referees hardly change, caching them with Redis is a low-hanging fruit, from which I expect to alleviate the potential negative performance impact.
- The Microservice architecture has potential for scaling: 
    - Availabilities: is probably the most heavily used service because it can be expected that referees use this the most when they use the tool. Also it  has read-write access. All-in-all for this service should exist the most instances.
    - Assignments: is only going to be used by a single admin, therefore no need to run many instances.
    - Fixtures: is essential for most operations in Availabilities and also when doing Assignments, so due to the massive scaling of availabilities, this service should also not be scale too low - however given the planned use of a Redis cache, the load shouldn't be too heavy.
    - Teams and Venues is similar to Fixtures, as both are needed as well when resolving Fixtures and Availabilities.
- Committing Assignments has to have some form of cross-service (Referees are assigned to Fixtures when Assignments are committed) transactional behaviour, which I am probably going to implement via Sagas.
- Using Kafka for broadcasting domain events, and implementing some form of idempotency on the domain level to avoid issues in case of duplicate event processing.
- Each service gets its own DB - but within the same postgres instance, which should be enough for the scaling requirements of this project.
- Using nginx as a reverse proxy to act as an API Gateway that hides the deployment details from the outside, so FE doesn't need to be adjusted, which is paramount for this undertaking. Also by using this API Gateway, microservices can call into each other via the same interface the UI uses.

### Running the Monolith

1. Make sure you have Rust installed, see [Rustup](https://rustup.rs/).
2. Make sure you have Docker with `docker compose` feature installed. 
3. Install `trunk` via `cargo install trunk` which is required for the Leptos frontend.
4. Switch to `nightly` Rust via `rustup toolchain install nightly` which is required for the Leptos frontend.
5. Add the wasm32 compile target via `rustup target add wasm32-unknown-unknown` which is required for the Leptos frontend to compile to WASM.
6. Start the Postgres DB by running `sh start_db.sh` from within the `./monolith` folder
7. Start the Backend by running `sh run.sh` from within the `./monolith` folder. The first time it compiles the backend from soure, which might take 1-2 minutes.
8. Start the Frontend by running `sh run.sh` from within the `./frontend` folder. The first time it compiles the frontend from sourvce, which might take 1-2 minutes. Once finished, it opens a browser for you and displays the starting page.

If you want to run the test suite, make sure you have a backend running and then simply run `sh tests.sh` from within the `./monolith` folder.

## Refactoring into Microservices

Currently I am working on refactoring the monolith into microservices, for which I started by writing [Architecture Decision Records](/microservice/ADR) for the overalle approach.

### Running the Microservices

1. Make sure you have Rust installed, see [Rustup](https://rustup.rs/).
2. Make sure you have Docker with `docker compose` feature installed. 
3. Install `trunk` via `cargo install trunk` which is required for the Leptos frontend.
4. Switch to `nightly` Rust via `rustup toolchain install nightly` which is required for the Leptos frontend.
5. Add the wasm32 compile target via `rustup target add wasm32-unknown-unknown` which is required for the Leptos frontend to compile to WASM.
6. Start all databases of the microservices by running `sh start_all_db.sh` from within the `./microservice` folder. 
7. Start Redis by running `sh start_redis.sh` from within the `./microservice` folder.
8. Start Kafka by running `sh start_kafka.sh` from within the `./microservice` folder.
9. Create the Kafka topic by running `sh create_domain_events_topic.sh` from within the `./microservice` folder. This will create a topic called `rustddd.events` with 2 partitions so that they will be consumed by 2 instances of each microservice.
10. Start Nginx by running `sh start_nginx.sh` from within the `./microservice` folder.
11. Start all microservices by running `sh run_all.sh` from within the `./microservice` folder. Note that this will start 2 instances of each microservice.
12. Start the Frontend by running `sh run.sh` from within the `./frontend` folder. The first time it compiles the frontend from sourvce, which might take 1-2 minutes. Once finished, it opens a browser for you and displays the starting page.

To stop all running microservices, you can run `sh stop_all.sh` from within the `./microservice` folder.

Each microservice has E2E tests, which can be run by calling `sh tests.sh` from within the respective microservice folder. Note that not each microservice needs all other services up and running but to make sure, simply run all of them (using `sh run_all.sh`). Also, given its an E2E test, you need to have Redis, Kafka and Nginx up and running.