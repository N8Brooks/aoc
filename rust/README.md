# Rust

Solutions for *[Advent of Code](https://adventofcode.com)* in rust.

## File Layout

Using an example date of December 1, 2022:

* The *year_2022* lib package of this workspace is for solutions in 2022.
* The *year_2022/src/lib.rs* references a *year_2022/day_01.rs* module with the solutions for December 1.
* If the module does not exist, the solution hasn't been written.

## Module Style

|Symbol|Description|
|------|-----------|
| part_1|Solution for part 1.|
| part_2|Solution for part 2.|
| tests|Testing module.|
|├  EXAMPLE|Multi-line, static `&str` example data.|
|├  INPUT|Static `include_str!` input data to `../testdata`.|
|└  part_1|Part 1 tests.|
|└  part_2|Part 2 tests.|

Tests make use of [test_case](https://docs.rs/test-case/latest/test_case/).

## Workflow

1. Add solutions with `cargo fmt` on save.
1. Check the output of `cargo test`.
1. Check the output of `cargo clippy`.
1. Commit using [conventional](https://www.conventionalcommits.org/en/v1.0.0/) styling.
