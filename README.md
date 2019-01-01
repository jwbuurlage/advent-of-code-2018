# Advent of Code 2018

These are solutions to the 2018 edition of [Advent of Code](https://adventofcode.com). This year, I have tried to implement all the solutions in Rust. This is definitely not best language to choose if you want to optimize for implementation time, but it did provide me an opportunity to write Rust almost daily for a while.

- I try to use the standard library as much as possible. I did end up using the following dependencies:
  - `nom` for parsing (it seems like a good library that I wanted to understand better which is why I ended up using it for some days. It certainly is not the fastest way to implement parsing code for a programming contest :))
  - `slab` for a doubly linked list implementation that I copied from stack overflow (used in Day 5 and Day 9).
  - `topological-sort` because I did not feel like implementing it myself (used in Day 7).
  - `image` for some visualization (only remaining use in Day 6)
  - `maplit` gives convenient macros for dictionaries (used in Day 12, Day 19 and Day 20)
- It is probably not the most idiomatic Rust code, but the solutions are generally relatively short.
- Some solutions require release builds to be reasonably fast. For most days, run using
  ```bash
  cargo run --release --bin dayXY < data/inputXY.txt
  ```
