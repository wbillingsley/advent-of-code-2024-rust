# Learning some Rust via Advent of Code

First up, these were done late - I only started around the 21st. Anyway, this Christmas I figured I'd try out Rust by doing Advent of Code puzzles.

That also means these are not necessarily the best solutions, rather a lot of them try out different language features. 

Roughly as follows (notes so that when putting together lecture slides I know which days to hit for examples):

Day:
1. First go at Rust. Lambdas. 
2. `let mut` with loop iterators
3. Rust's regular expressions.
4. `chars()` and `nth()` as Rust's way of indexing into characters in a string; `usize` vs `i64` vs `u64`
5. Mix of imperative and functional style
6. `clone()`
7. Lifetime parameters, generics
8. (not a lot extra in this one)
9. enum, `#derive` directives
10. (not a lot extra in this one)
11. `BigUint`, Rust's BigInt
12. `VecDeque`, Rust's vector queue
13. (not a lot extra in this one)
14. `struct`, `impl`, `impl ops`, operator overloading, etc
15. `Eq, PartialEq` derives. Implementing `::from` for structs. `..` notation, shorthand notation.
16. `const` arrays, `Rc`, accepting lambdas via `impl Fn`. Uses a home-grown immutable list type as the `Rc` example and fold as the `impl Fn` example.
17. Part 2 has Rust's bitwise operators.
