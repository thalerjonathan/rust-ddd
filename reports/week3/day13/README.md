# Day 13 + 14: Implementing Assignments

The plan for today was to implement the *Assignments* feature, which allows for assigning *Referees* to *Fixtures*.

According to the User Stories this should work via a *staging* process: the Admin can freely assign *Referees* to *Fixtures* without having to worry about conflicts, i.e. two *Referees* being assigned to the same *Fixture* or *Referees* getting notified. Only when the Admin *commits* them, the *Assignments* are checked for conflicts and then fixed and Referees notified.

Note that the *Availability* UI is not finished yet, so there is still work to be done there. However I wanted to get started on the backend logic for the *Assignments* feature already so that I can then continue with the frontend for both features in the remaining day of th week 3.

The first step is to make a change to the existing DB schema, adding first and second *Referee* to the *Fixture* entity, which also has to be reflected in the domain model and the whole code base.
Then I can start implementing the *Assignments* feature, driven by the E2E test scenario, just as I outlined it yesterday in Day 12.

## Results

The first step, adding a first and second *Referee* to the *Fixture* entity, turned out to be much more work than expected because I ran into some annoying problems with SQLx regarding nullable types and enums. Ultimately this all boiled down to how to correctly map the enums and the nullable fields, with SQLx having still some issues with that, therefore in some cases it needs explicit help from the developer.

However the rest was straightforward, with Cursor providing excellent support - as usual.

## Conclusions

Due to higher complexity, more refactoring work and interview preparation I had to stretch the Assignments implementation over 2 days, and leave the UI implementation for tomorrow. 

All in all it was pretty straightforward, I only ran into small unexpected issues with SQLx, where again, I didn't get useful help from Cursor but needed to dig my way into some internet forums and stackoverflow.
