# Day 6

Due to limited time due to a job interview today, the plan for today was simply to implement the *Venue* part.

## Results

The *Venue* part is very similar to what already exists for the *Referee* part, so I simply asked Cursor to copy the existing code and adapt it to the new use case.

I wasn't very happy with what the Cursor composer generated - it didn't look bad, and the code for the repo looked actually quite good, but into which files it placed them I wasn't very happy with, also some details were missing. I decided to not go with the generated code and rely on the contextual awareness of Cursor to generate the code and me directing it to place it in the right file.

This time I started in a TDD manner, writing the integration tests first, then following with the REST implementations, and finally the whole application service, aggregate, repository and DB adapter. With Cursors context awareness, I had this done within around 30 minutes.

The next step was to implement the leptos part, where I simply followed the existing referee code and adapted it to the new use case, which took no longer than 15 minutes.

## Conclusions

Today wasn't particularly spectacular - I just needed to "get things done" for the venues - there was nothing "new" in there, so using Cursor to generate the code guided by me was exactly the perfect use case. Overall it took me around 1 hour, which is pretty fast - without Cursor I think I would have not taken VERY much longer, because I used the referee code as template, so it would have been copy-paste and adapt as well, but still I think with Cursor I could shave off at least 15 minutes, which is always nice.