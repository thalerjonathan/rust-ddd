# Day 10 + 11: Transactional Boundaries

The plan for todday was to refactor the Repositories to use a Transaction, instead of having a single connection pool (a fact that I already mentioned in [Day 9](../day9/README.md)), so that we can properly enforce transactional boundaries across multiple Repositories. This is a fundamental requirement for DDD, because we want to make sure that we never have a situation where only some of the Aggregates/Entities in a *Bounded Context* are updated, while others are not due to some exceptions/errors.

## Results
After some researching and experimenting it became apparent that in Rust this is a very non-trivial problem.

In theory the application service layer knows about transactional boundaries, that is, it knows which Use Case needs transactional guarantees and where the transaction starts and where it should end. However, the problem with Rust is that we don't have anything like annotational transaction demarcation as we have in Spring. Therefore a naive approach would be to 
pass the connection pool to the application services and let them deal with transactions. This is simple but we would be tightly coupling the services to the DB implementation, which would make it impossible to test the application services in isolation without a real database. Because the application services drive the Use Cases/Stories, being able to test application services in isolation is so fundamentally important that we cannot compromise on this. I achieved this so far by instantiating the respositories in the REST handlers (ports), and passing them to the application services: this way the application services remain agnostic to the DB implementation details, because the interact via a trait (interface) instead of concrete implementations. In my implementation so far the Repositories all receive a reference to the connection pool, which is then passed to the query as executor. All of this works perfectly fine, because the SQLx connection pool is intended to be passed around which is also indicated by its [ability to get cloned](https://docs.rs/sqlx/latest/sqlx/pool/struct.Pool.html):

```
Pool is Send, Sync and Clone. It is intended to be created once at the start of your application/daemon/web server/etc. and then shared with all tasks throughout the process’ lifetime. How best to accomplish this depends on your program architecture.

... 

Cloning Pool is cheap as it is simply a reference-counted handle to the inner pool state. When the last remaining handle to the pool is dropped, the connections owned by the pool are immediately closed (also by dropping). PoolConnection returned by Pool::acquire and Transaction returned by Pool::begin both implicitly hold a reference to the pool for their lifetimes.
```

So far so good, but as already mentioned, this design has a fundamental flaw: it doesn't allow for multiple repository queries to span a single transaction. The reason for that is, that with the current design, although the method implementations of the respositories have access to the *same* connection pool, they don't share the same transaction, because they have no other option than to pass the connection pool as *Executor* to the query methods, which would always fetch a fresh connection from the pool, and thus not be part of the same transaction.

This can cause fundamental issues, leaving us with an inconsistent data state in the database, for example when we have two respositories where the update to the first one succeeds, but the second one fails for some reason - in this case we would be in an inconsistent state.

My initial idea was that instead of a connection pool to pass a reference to a *Transaction* object to the concrete *Repositories*. The *Respository* can then pass the *Transaction* to the query methods, so that they are executed in the context of the same transaction. 

This works fine if you are only using a single repository, however as soon as you have multiple repositories in a single application service, this won't work. The reason for that is that the *Transaction* needs to be passed as ```mut``` to the query execution, which also means it needs to be borrowed mutable when instantiating the respective *Respository*  in the REST handler. This by itself would work, however as soon as you want to created multiple *Respositories* (which is not unusual) you would need to borrow the *Transaction* mutably **multiple times at the same time** which Rust fundamentally doesn't allow, which is stated very clearly in the [Rules of References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references):

```
At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.
```

Therefore this approach seemed to be hitting a dead end - even worse it seems that SQLx fundamentally does not allow for this kind of transactional behaviour, which seemed to be confirmed by this [issue](https://github.com/launchbadge/sqlx/issues/2312) on the SQLx repository. As a note: sharing a single connection across multiple repositories is not an option, because of the same problem with *Transaction* objects: the connection would need to be passed as ```mut``` to the respective repository, which would again lead to the same problem of being unable to borrow it mutable multiple times.

The next thing I tried was to resort to `unsafe` code using [`UnsafeCell`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html) and mutating public static mutable variables. This also didnt work due to Rust's memory safety guarantees.

My final idea was to pass the Tx as argument to each of the repository methods, but via a generic trait, so the actual type of the *Transaction* can vary between the different implementations. However, I ran into another problem, with the compiler reporting:

```
implementation of `VenueRepository` is not general enough
```

This is a very weird error, because the implementation is obviously more general than what the compiler expects. Adding a phantom lifetime parameter to the trait didn't help - I then found that there are some [issues with the trait solver](https://github.com/rust-lang/rust/issues/101794) and its been written new. However when compiling with `RUSTFLAGS="-Znext-solver"` tokio didn't compile, so this was a dead end.

The solution to the problem was to parameterise the trait not via a generic type but via a associated type - this way we don't run into the problem that the `impl` potentially holds a lifetime, which was the cause of the error. The downside is that we now are forced to ALWAYS use a Transaction, even if a simple read-only query is executed. It is possible to implement a separate "non-transactional" repository, but this is beyond the scope of this project. A benefit of this approach however is that read-only operations can be enforced on the REST handler level by never committing the transaction, which results in an automatic rollback, as soon as the handler returns (either via success or error path). However this means that the REST handler has now much more knowledge about the Domain, which DDD purists would probably not like - an interesting idea would be to somehow extract the transactional wrapper into a separate component that wraps around the application service layer/or becomes part of it in an "annotational" way via Rusts macros.

To wrap up the day I wrote tests for the *Fixture* application service using the `mockall` crate for easy mocking of the repositories. Also, I made some UX changes to the UI such as disabling "change" buttons in case a Fixture is cancelled.

## Conclusion
A big takeway for me is that language features heavily influence how to exactly implement a certain concept. 

Before I embarked on the task of refactoring towards transactional boundaries, I had the feeling that it won't be trivial due to Rusts way of dealing with references. However, I didn't expect it to be as hard as it actually was. I am happy with my current design, because it allows me to keep the application services agnostic to the DB implementation details, while still being able to enforce transactional boundaries, while retaining the benefits of having the Tx being rolled back automatically in case of an error. With global mutable state this would become horribly complicated and error prone (and we haven't even started to talk about concurrency issues).

Regards the `not general enough` error I ran into, Cursor / LLMs were of no help there: this comes as no surprise because the problem is very complex and Cursor/LLMs don't really *understand* code, so working out those problems requires a deep understanding of the language and can only be resolved by going deep, thinking hard and experimenting (with some inspiration from the internet).

A rather funny experience was when Cursor suggested some Chinese/Japanese text when typing this README - see the photo below
![Cursor speaking Chinese](cursor_chinese.jpg "Cursor speaking Chinese")
