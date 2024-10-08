# Day 7

Today is gonna be another very short day, as I have another job interview upcoming in the afternoon and I want to keep my mind fresh for that.

The plan is to simply implement the *Team* part.

## Results

I followed the same approach as for the other parts and used Cursor to generate the code. I also started with the integration tests, then the REST implementation, and finally the application service, aggregate, repository and DB adapter.

I started with adding the *Team* table to the init.sql, and then proceeded to the integration tests.
With the integration tests done, I started with the functions in shared project, that fetch teams and create teams via HTTP/REST.
Then the actual endpoints were added in the backend project with their respective handlers.
Then followed the Team Aggregate, repository and DB adapter.
Finally the application service and their usage in the REST handlers.
This took me less than 30 minutes (including the README for Day 7).

## Conclusions

As previously, Cursor proved to be very useful for generating the code and I had this stuff done in less than 1 hour.