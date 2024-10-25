# Day 19: Adding Caching via Redis

Due to time limitations I was not able to implement the caching via Redis on Day 18, therefore the plan for today was to do that.

Again I don't have too much time today, due to interview prep for next week.

## Results

Adding caching via Redis was pretty straightforward. The only thing that was tricky was to get the async code working with axum and generalising the "run_cached" function to work with any DTO.

## Conclusions

Next week (5) the first step is to add Kafka to emit Domain Events that can be used to invalidate the cache, e.g. in case the club of a Referee has changed.

