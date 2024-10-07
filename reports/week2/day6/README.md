# Day 6

Due to limited time due to a job interview today, the plan for today was simply to implement the *Venue* part.

## Results

The *Venue* part is very similar to what already exists for the *Referee* part, so I simply asked Cursor to copy the existing code and adapt it to the new use case.

I wasn't very happy with what the Cursor composer generated - it didn't look bad, and the code for the repo looked actually quite good, but into which files it placed them I wasn't very happy with, also some details were missing. I decided to not go with the generated code and rely on the contextual awareness of Cursor to generate the code and me directing it to place it in the right file.

This time I started in a TDD manner, writing the integration tests first, then following with the REST implementations, and finally the whole application service, aggregate, repository and DB adapter.

## Conclusions