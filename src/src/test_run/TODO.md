* Tests should run in parallel when possible
  * ...unless they're main tests, in which case they need to run on the main thread
* So ```discovery``` needs to generate two lists: the normal tests and the main tests
* Since the main thread's split between displaying test start/stop and running main tests,
there needs to be some way to evenly split its job time
  * A library probably already exists for this!
