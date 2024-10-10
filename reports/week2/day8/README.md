# Day 8

Today the plan is to implement the *Fixture* part - the arguably most complex part of the system.

The reason for the complexity lies in the fact that a *Fixture* contains a *Venue* and 2 *Teams*, both being Entities - therefore we need an efficient way to fetch them when loading *Fixtures*. 
Also a *Fixture* allows to change date, venue, be cancelled and when creating a new one the following constraints have to be respected:
- Both teams have to be different.
- No other *Fixture* is scheduled at the same venue and time.
- The teams cannot have other *Fixtures* scheduled at the same time.
- Check if date is in the future.

The challenge is to how to implement this via DDD.

## Results

I followed my usual approach of first creating the integration tests, then the shared classes, then the REST implementation, then the aggregate and finally the repository and DB adapter.
I also implemented the *Fixture* component in the frontend.

## Conclusions

Generating the SQL for the *Fixture* table was done easily by Cursor - but what was more impressive was when generating INSERT it was clever enough to "understand" which IDs it should fill and generated 3 correct inserts for the 3 existing teams in 3 different venues at 3 different times. However Cursor generated UUID v4s in the wrong format, so I had to correct them manually.

I only managed to implement loading and creating Fixtures, but I didn't manage to implement the update and cancel features. Also, while implementing the backend I realised that passing a connection pool to the repositories is not a good idea because we want to be able to enforce the same transaction for the operations on the Fixture and its related entities (Venue and Teams) - and with fetching a new connection for each repository we lose the transactional guarantee/flexibility.

I am going to continue tomorrow working on the remaining features for the *Fixtures* part.