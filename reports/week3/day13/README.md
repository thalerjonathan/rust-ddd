# Day 13: Implementing Assignments

The plan for today was to implement the *Assignments* feature, which allows for assigning *Referees* to *Fixtures*.

According to the User Stories this should work via a *staging* process: the Admin can freely assign *Referees* to *Fixtures* without having to worry about conflicts, i.e. two *Referees* being assigned to the same *Fixture* or *Referees* getting notified. Only when the Admin *commits* them, the *Assignments* are checked for conflicts and then fixed and Referees notified.

Note that the *Availability* UI is not finished yet, so there is still work to be done there. However I wanted to get started on the backend logic for the *Assignments* feature already so that I can then continue with the frontend for both features in the remaining day of th week 3.

The first step is to make a change to the existing DB schema, adding first and second *Referee* to the *Fixture* entity, which also has to be reflected in the domain model and the whole code base.
Then I can start implementing the *Assignments* feature, driven by the E2E test scenario, just as I outlined it yesterday in Day 12.

## Results

## Conclusions