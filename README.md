# Quicksort Benchmark

This is a quick benchmark of some quicksort implementations in Rust.
The goal was to measure the overhead of immutable data structures in Rust
in a real-world scenario.

## Results

The following results were obtained on a 2021 MacBook Pro M1 and 32 GB of RAM.

| Implementation      | Time (s) |
|---------------------|----------|
| mutable             | 0.25     |
| immutable           | 0.35     |
| parallel immutable  | 19.06    |

As you can see, the immutable implementation is about 40% slower than the
mutable one. The parallel immutable implementation is about 76 times slower than
the mutable one and 54 times slower than the immutable one.
That is because the overhead of spawning threads is much higher
than the overhead of copying the data sequentially.

That said, I did not expect the overhead to be that high, so perhaps there is
something wrong with my implementation. If you have any suggestions, please
open an issue or a pull request.