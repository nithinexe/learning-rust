Basically variables are immutable in rust. To make them mutable you need to explicityly specify `mut` keyword to a variable.

So the problem here: Problem: You're parsing a network packet header. The total length is fixed, but the payload size might change.

Exercise: Try removing the `mut` and reassigning the `payload_size` to see the Rust's compiler error.
