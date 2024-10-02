# Day 3: Bootstrapping the UI with Leptos

On Day 3 the plan was to bootstrap the UI with Leptos. What I wanted to arrive at was a basic UI for the Referee endpoints, that is, create a new Referee, list details of a single Referee and list all existing Referees.


## Results

I started off with telling Cursor to "implement the leptos UI for the Referee endpoints". It came up with a some basic leptos code, but it was too generic and not tailored to the specific requirements of the Referee endpoints - I wanted to see if Cursor could generate the code for the specific UI components for the Referee endpoints. With a more specific prompt "generate a web ui in the frontend folder using leptos for the referee endpoints found in the backend folder", I got the result I wanted, even including the HTTP requests to the backend.

I then ran into an error "RelativeUrlWithoutBase" when fetching referees from the backend which I struggled a long time to fix. The weird thing was that the HTTP request went through successfully but the browser console showed the error. For some reason it complained about the "http://localhost:3001/referees" URL being relative even though it was clearly absolute. 

## Conclusion

I was very impressed by Cursor's ability to generate the basic leptos code, and it was fun to see that it could generate the HTTP requests too. However, due to the "RelativeUrlWithoutBase" error I was not able to get the HTTP requests to work, and I ran out of time for the day, so I had to leave the day with only a partial result.
