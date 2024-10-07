# Day 4: Continued and finished bootstrapping the UI with Leptos

After the small set-back from Day 3, where I ran into an error, on Day 4 I continued with the UI bootstrapping. 

## Results

I pretty quickly found the cause of the error from Day 3: I have forgotten to enable CORS on the backend side, so after adding it there, fetching referees from the backend worked perfectly fine.

After I have fixed the CORS problem and implemented "show all referees" as well as "show details of a referee" functionality, I continued with adding the ability to add new referees. For this I asked Cursor to come up with functionality to do that. I compared the code it produced with the code from the documentation, to understand what it wants to do - it was pretty close, so I went with it. The code worked only with minor changes (slightly refining REST request) straight out of the box.

I then spent a bit of time to make the "create referee" functionality "reactive": when a referee was created successfully in the backend and we receive a successful response, we want to update the list of referees in the UI without re-fetching all of them. To do this, I had to make the frontend code reactive, which I did via signals and effects, and updating the list of referees via a signal. I am not sure if my current approach is idiomatic for Leptos, but it works, and is good enough for this project.

Next, I wanted to refactor the code into DTOs that are shared between the frontend and backend - something that is fundamental, to avoid inconsistencies between the two, and that is super easy given we are using Rust for both. I wasn't parcitularly successful in telling Cursor to produced DTOs - it either didn't get the context or ended up writing TypeScript code. Therefore I decided to do it by hand, which was very easy and then Cursor picked up the context very quickly and supported me with perfect suggestions.

Another thing I did was to add a *Club* text field to the Referee, which should indicate the Club the Referee is currently working for, and which should be changeable via a form in the UI. I explicitly modelled it as a "free form" text field, since I didn't want to go to the lengths of representing Clubs as separate entities in the application. The reason I added it, was to have a case for changing a field of an Entity/Aggregate without changing its identity - something fundamentally important in DDD. Adding the club field was extremely easy with the help of Cursor, because due its contextual awareness, it correctly inferred everwhere immediately what I wanted to do and produced the code for it.

## Conlusion

I was pretty impressed that Cursor generated code for creating a new referee that worked basically out of the box. Also, the context awareness of Cursor is just insane: adding the club field basically didnt require any typing, just using "TAB" to apply the suggestions made by Cursor, which were all correct.

However, despite the impressive results from above, another one of todays learning lesson was that despite how impressive the capabilities of Cursor (LLMs) to generate code are, you still need to have a good understanding of the specific framework (and language) you are using - in this case Leptos - and of the fundamentals of HTTP / web and CORS. The LLMs didnt tell me that my problem was CORS, which I only realised myself after some time, because I simply had forgotten this detail.

All in all, this concludes the bootstrapping of the UI, which I am pretty happy about.