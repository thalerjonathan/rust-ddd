# Day 4: Continued and finished bootstrapping the UI with Leptos

After the small set-back from Day 3, where I ran into a persistent error, on Day 4 I continued with the UI bootstrapping. 

## Results

I pretty quickly found the cause of the error from Day 3: I have forgotten to enable CORS on the backend side, so after adding it there, fetching referees from the backend worked perfectly fine.

After I have fixed the CORS problem and implementing "show all referees" as well as "show details of one referee" functionality, I continued with adding the ability to add new referees. For this I asked Cursor to come up with functionality to do that. I compared the code it produced with the code from the documentation, to understand what it wants to do - it was pretty close, so I went with it. The code worked only with minor changes (slightly refining REST request) straight out of the box.

I then spent a bit of time to make the "create referee" functionality "reactive": when a referee was created successfully in the backend and we receive a successful response, we want to update the list of referees in the UI. To do this, I had to learn how to make the frontend code reactive, which in retrospect is pretty straightforward once you get the hang of it.

Afer creating Referees worked, I wanted to refactor the code into DTOs that are shared between the frontend and backend. TODO: use Cursor for this

Another thing I did was to add a *Club* string field to the Referee, which should indicate the Club the Referee is currently working for, and which should be changeable via a form in the UI. I am explicitly modelling it as a "free form" text field, since I don't want to go to the lengths of representing Clubs as separate entities in the application. The reason I am adding it, is to have a case for changing a field of the referee, without changing its identity - something fundamentally important in DDD.

## Conlusion

Thus todays learning lesson was that despite how impressive the capabilities of Cursor (LLMs) to generate code are, you still need to have a good understanding of the specific framework you are using - in this case Leptos - and of the fundamentals of HTTP / web and CORS. The LLMs didnt tell me that my problem was CORS, which I only realised myself after some time. Also, regarding using Leptos `create_resource` I only realised that I have to use it after I looked deeper into the leptos documentation.

Pretty impressive though was that Cursor generated code for creating a new referee that worked basically out of the box.

