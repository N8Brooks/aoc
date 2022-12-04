# rust

Solutions for *[Advent of Code](https://adventofcode.com) in rust.

## File Layout

Using an example date of December 1, 2022:

* The *year_2022* lib package of this workspace is for solutions in 2022.
* The *year_2022/src/lib.rs* references a *year_2022/day_01.rs* module with the solutions for December 1.
* If the module does not exist, the solution hasn't been written.

## Module Style

 part_1      Returns the solution for part 1 from the input as `&str`.
 part_2      Returns the solution for part 2 from the input as `&str`.
 tests       Testing module.
├  EXAMPLE   A multi-line `const` string reference for the example data for the day.
├  INPUT     A `const` string reference from the `include_str!` macro to *../testdata*.
└  part_1    Tests defined with the [test_case](https://docs.rs/test-case/latest/test_case/) crate for `problem_1`.
└  part_2    Tests defined with the [test_case](https://docs.rs/test-case/latest/test_case/) crate for `problem_2`.

## Workflow

1. Add solutions.
1. Check the output of `cargo test`.
1. Check the output of `cargo clippy`.
1. Commit using [conventional](https://www.conventionalcommits.org/en/v1.0.0/) styling.
