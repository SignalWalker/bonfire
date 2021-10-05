# CapTP-rs

A Rust implementation of [CapTP](http://erights.org/elib/distrib/captp/index.html).

## Memory Management

In a normal Rust program, objects are garbage collected in one of two ways:
* Statically, using lifetimes (which correspond to a specific scope, up to the life of the program)
* Dynamically, using reference counting (Rc, Arc, etc.)

In a multithreaded Rust program, memory management can still rely on lifetimes and reference counting by encapsulating threads in scopes (this is how Rayon works). If a thread panics, the program exits with a panic as well, or the thread is cleaned up and returns an Err(_).

However, these have problems in a distributed Rust program, because instances may come and go asynchronously. The E language's solution is a distributed garbage collector[1], but I would prefer something Rustier. Maybe lifetimes can somehow be distributed? Or, considering that distributed programming is very similar to multithreaded programming, something like reference counting might work -- reference counters, though, rely on a central counter stored somewhere, so that might not be viable for multiple reasons.

At worst, we'll just implement E's cyclic garbage collector -- this, apparently, isn't possible in Goblins due to an inability to control Racket's garbage collector at that level, but Rust's can be arbitrarily manipulated in unsafe blocks.

## References

1. [Distributed Programming with CapTP](https://docs.racket-lang.org/goblins/captp.html)
2. [The Electric Communities Distributed Garbage Collector](http://erights.org/history/original-e/dgc/)
3. [What is CapTP?](https://spritelyproject.org/news/what-is-captp.html)
4. [Content Addressed Descriptors and Interfaces with Spritely Goblins](https://dustycloud.org/tmp/interfaces.html)
5. [Goblins: A Transactional, Distributed Actor Model Environment](https://docs.racket-lang.org/goblins)
6. [A Security Kernel Based on the Lambda Calculus](http://mumble.net/~jar/pubs/secureos/secureos.html)

## See Also

* [cap-std](https://crates.io/crates/cap-std) :: capability-based version of rust-std
* [SwingSet](https://github.com/Agoric/agoric-sdk/tree/master/packages/SwingSet)
* [Cap'n Proto](https://capnproto.org/)
* [Waterken](http://waterken.sourceforge.net/)