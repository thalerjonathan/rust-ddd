# Day 8

Today the plan is to implement the *Fixture* part - the arguably most complex part of the system.

The reason for the complexity lies in the fact that a *Fixture* contains a *Venue* and 2 *Teams*, both being Entities - therefore we need an efficient way to fetch them when loading *Fixtures*. 
Also a *Fixture* allows to change date, venue, be cancelled and when creating a new one the following constraints have to be respected:
- Both teams have to be different.
- No other *Fixture* is scheduled at the same venue and time.
- The teams cannot have other *Fixtures* scheduled at the same time.
- Check if date is in the future.

The challenge is to how to implement this via DDD.

I followed my usual approach of first creating the integration tests, then the shared classes, then the REST implementation, then the aggregate and finally the repository and DB adapter.

## Results

## Conclusions

Generating the SQL for the *Fixture* table was done easily by Cursor - but what was more impressive was when generating INSERT it was clever enough to "understand" which IDs it should fill and generated 3 correct inserts for the 3 existing teams in 3 different venues at 3 different times. However Cursor generated UUID v4s in the wrong format, so I had to correct them manually.