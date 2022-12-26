# Rust

This project contains solutions to the [Advent of Code](https://adventofcode.com) programming challenges, written in Rust.
Advent of Code is a series of small programming puzzles for a variety of skill levels, released every day leading up to Christmas in December.

## Building and Running Solutions

To build and run the solutions, you will need to have the Rust programming language and its package manager, Cargo, installed on your system.
You can find installation instructions for Rust at https://www.rust-lang.org/tools/install.

To build and run the solutions, navigate to the \`rust\` directory of the project and run the following command:

```rs
cargo test
```

This will build and run the solutions for all the available days.
You can also run the tests for a specific year's solutions by navigating to the \`year_YYYY\` directory and running the same command.

To run the tests for a specific day like Day 1, navigate to the year_YYYY directory and use the following command:

```rs
cargo test -- day_01
```

Solutions are tested using the example(s) provided in the problem description, as well as the input data located in the ../testdata directory.
These tests make use of the [test_case](https://docs.rs/test-case/latest/test_case/) crate.

## Contributing

Contributions are welcome! If you have a solution for a day that is not yet included, or if you have an improvement to an existing solution, please feel free to submit a pull request.

Before submitting a pull request, please make sure to:

- Follow the code style and conventions used in the project.
- Run cargo fmt and cargo clippy to format and lint your code.
- Add tests for your code, using the test_case crate if applicable.

## Code Layout and Organization

The code for this project is organized into a workspace, with one crate for each year of Advent of Code.
Within each crate, the code is organized by day, with one module for each day.

The layout of each day's module follows the pattern shown in the table below:

| Symbol    | Description                                        |
| --------- | -------------------------------------------------- |
| part_1    | Solution for part 1.                               |
| part_2    | Solution for part 2.                               |
| tests     | Testing module.                                    |
| ├ EXAMPLE | Multi-line, static `&str` example data.            |
| ├ INPUT   | Static `include_str!` input data to `../testdata`. |
| ├ part_1  | Part 1 tests.                                      |
| └ part_2  | Part 2 tests.                                      |

Each day's module follows this same pattern, with the relevant code and tests for that day.
For example, the code for Day 1 of Year 2022 would be located in the year_2022/day_01 module.
If the module does not exist, the solution for that day has not yet been written.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
