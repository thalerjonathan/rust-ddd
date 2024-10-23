# Day 18: Fixtures Microservice and Caching with Redis

The plan for today was to create the Fixtures microservice. Due to the fact that the Fixtures also require data from the Teams and Venues microservices, an architectural decision was made to use Redis as a caching layer, therefore the plan for today was to also set up Redis and add some basic caching logic to the Fixtures microservice.

## Results

A fundamental change in the Aggregate of the Fixture was that instead of full Venue, Team and Referee objects, only the IDs are stored in the Fixture aggregate. This way, the Fixtures microservice is truly independent from the Teams and Venues microservices. However this meant we needed some way to "resolve" the IDs to full objects, which I decided to do in the REST handlers. I added "Resolver" traits, which are implemented by the shared library and can be used to resolve the IDs to full objects, returned as DTOs. The "Resolver" implementations are straightforward, simply fetching the data from the REST interface of the Teams and Venues microservices.

As a consequence in the E2E tests, the Teams and Venues microservices had to be started before running the tests.

Also I decided to be pragmatic in the E2E tests and DELETE the table in the Teams and Venues microservices by connecting to the respective database directly. This is a fundamental violation of the Microservices architecture, however it simplifies the E2E tests a lot. The alternative would be to provide some DELETE endpoints in the Teams and Venues microservices, which I considered worse, as it adds semantically wrong (there is no business reason in the requirements to delete Venues or Teams) endpoints to the Teams and Venues microservices, and offers "dangerous" functionality, that could be abused.

## Conclusions

Most of the work was refactoring over all projects, as I had to move some types around and decided also to rename some projects to match the new structure.

I did not manage to add Redis caching yet, as the refactoring and implementation of the Fixtures microservice took more time than expected.