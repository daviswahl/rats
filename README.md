# Rats

Rats is an experimental, type-level functional programming library for Rust,
based heavily on Scala's Cats (which itself is based heavily on Scalaz). There have been a few explorations in this space,
but I believe Rats takes it much further than was previously possible.

Rats has a few goals: 

1. Implement functional abstractions as close to zero-cost as can be achieved, while still maintaining the usefulness 
of these abstractions. This is a delicate balance.
2. Explore functional programming in the context of Rust. 
3. Learn more about FP, and get better at Rust.

At the moment, Rats relies on a non-zero cost embedding of higher-kinded types. For this reason, Rats is probably not 
appropriate for performance critical programs. However, it does enable some powerful abstractions that might be useful
in less performance-critical applications. For more on the HKT embedding and how it works, see `lifted.rs`.

Due to the performance constraints, Rats will likely only be of interest to Rust programmers curious about functional 
programming, and to functional programmers who are curious about Rust. At the moment, it is a single person's labor of 
madness and unemployment, but there are lots of areas where Rats can be expanded, and I'd be very happy to accept 
contributions. 


## Contributing

TODO

## Thanks

A huge thanks to the Cats project and contributors, who are responsible for everything I know about type-level
functional programming. 

## And Finally

Rats is dedicated to Ada, Beorn, Basil, Elsie, Gracie, Rosie, Sally, Two-of-Three and Yuri. Their lifetime's were 
much too short.

