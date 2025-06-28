# Rustdoku - Sudoku Solver

This is a fast Sudoku solver implemented in Rust using **bitboard representations**.

## Usage

To build and use this solver, you need **cargo**.

### Running the Solver
To run the solver with a Sudoku puzzle, use:
```sh
cargo run ..2..............1....23.4....................7...28.......62..5..1.....91.....8.
```
inside the repository or build the porject using `cargo build` and execute the executable using:

```sh
./target/debug/rustdoku ..2..............1....23.4....................7...28.......62..5..1.....91.....8.
```
.

### Example Puzzle

```sh
time ./target/debug/rustdoku ..2..............1....23.4....................7...28.......62..5..1.....91.....8.
*-----------------------*
| . . 2 | . . . | . . . |
| . . . | . . . | . . 1 |
| . . . | . 2 3 | . 4 . |
|-------+-------+-------|
| . . . | . . . | . . . |
| . . . | . . . | . . . |
| . 7 . | . . 2 | 8 . . |
|-------+-------+-------|
| . . . | . . 6 | 2 . . |
| 5 . . | 1 . . | . . . |
| 9 1 . | . . . | . 8 . |
*-----------------------*
*-----------------------*
| 6 4 2 | 9 7 1 | 5 3 8 |
| 7 9 3 | 4 8 5 | 6 2 1 |
| 1 8 5 | 6 2 3 | 7 4 9 |
|-------+-------+-------|
| 2 6 9 | 7 1 8 | 4 5 3 |
| 8 5 4 | 3 6 9 | 1 7 2 |
| 3 7 1 | 5 4 2 | 8 9 6 |
|-------+-------+-------|
| 4 3 7 | 8 9 6 | 2 1 5 |
| 5 2 8 | 1 3 4 | 9 6 7 |
| 9 1 6 | 2 5 7 | 3 8 4 |
*-----------------------*

real    0m0.005s
user    0m0.002s
sys     0m0.003s
```

A puzzle is encoded as a string by reading the Sudoku board from left to right, row by row, from top to bottom, where each `.` represents an empty cell and numbers `1-9` represent pre-filled values.

## Features
- Uses **bitboards** for fast board representation and candidate tracking.
- Implements **backtracking**.
- Optimized search for the next best cell to solve first.
- **bitwise operations** to speed up solving.

## Benchmark
Results for set of 2365 sudoku puzzles:
```sh
real    0m0.077s
user    0m0.073s
sys     0m0.004s
```