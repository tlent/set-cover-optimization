# Set Cover Optimization

This repository offers solutions to the
[set cover optimization problem](https://en.wikipedia.org/wiki/Set_cover_problem)
through four different programs, each in its own directory.

## Programs

- **Python, C, and Rust:** These programs share a common backtracking algorithm,
  with the Rust version being the most optimized.

- **Z3:** This version utilizes the
  [Z3 theorem prover](https://github.com/Z3Prover/z3) through the `z3-solver`
  Python package.

## Algorithm Overview

The programs employ a backtracking algorithm to find an exact minimum set cover
for a given set of sets. The algorithm iteratively selects different
combinations of sets until the minimum set cover is identified. For each
iteration the algorithm incorporates the following checks to optimize the
process:

- **Subset Elimination:** If a set is a subset of another set, the subset is
  excluded from consideration. This eliminates redundant sets, as choosing a
  subset covers fewer elements for the same cost.

- **Singleton Element:** If an uncovered element is unique to a single set, that
  set is selected as it is necessary for any valid set cover.

- **Set Selection:** If neither of the above conditions is met, the algorithm
  considers selecting or omitting a set. It prioritizes sets with the most
  uncovered elements, as they are more likely to be part of the minimum set
  cover.

The Rust program has an additional optimization:

- **Upper Bound**: Maintain an upper bound on the minimum set cover size
  encountered so far. If at any point the current partial solution exceeds this
  bound, prune that branch of the search space

## Runtime Comparison

The table below presents a runtime comparison for the test cases across the C,
Rust, and Z3 programs. All test cases were run on an M2 Macbook Air.

| Test Case    | C         | Rust       | Z3        |
| ------------ | --------- | ---------- | --------- |
| s-rg-8-10    | 4.00 µs   | 3.21 µs    | 25.79 ms  |
| s-X-12-6     | 4.00 µs   | 4.75 µs    | 777.96 µs |
| s-k-20-30    | 28.00 µs  | 6.75 µs    | 2.22 ms   |
| s-k-30-50    | 114.00 µs | 19.58 µs   | 1.41 ms   |
| s-rg-31-15   | 36.00 µs  | 20.58 µs   | 675.92 µs |
| s-rg-40-20   | 49.00 µs  | 19.71 µs   | 485.90 µs |
| s-k-40-60    | 150.00 µs | 16.50 µs   | 1.71 ms   |
| s-k-20-35    | 96.00 µs  | 32.25 µs   | 1.55 ms   |
| s-rg-63-25   | 298.00 µs | 111.83 µs  | 995.87 µs |
| s-k-30-55    | 490.00 µs | 115.75 µs  | 2.08 ms   |
| s-rg-118-30  | 2.78 ms   | 246.71 µs  | 819.92 µs |
| s-rg-109-35  | 919.00 µs | 252.96 µs  | 784.16 µs |
| s-k-35-65    | 1.53 ms   | 310.58 µs  | 2.61 ms   |
| s-rg-155-40  | 4.39 ms   | 1.01 ms    | 1.26 ms   |
| s-rg-197-45  | 5.93 ms   | 1.43 ms    | 1.22 ms   |
| s-rg-245-50  | 19.19 ms  | 4.17 ms    | 1.83 ms   |
| s-k-40-80    | 55.83 ms  | 18.57 ms   | 4.84 ms   |
| s-k-50-95    | 142.66 ms | 20.40 ms   | 44.18 ms  |
| s-rg-413-75  | 248.37 ms | 36.10 ms   | 6.14 ms   |
| s-k-150-225  | 574.56 ms | 125.76 ms  | 54.49 ms  |
| s-k-50-100   | 1.73 s    | 400.80 ms  | 19.48 ms  |
| s-rg-733-100 | 7.60 s    | 1.14 s     | 2.18 s    |
| s-k-200-300  | 40.60 s   | 7.66 s     | 4.32 s    |
| s-k-100-175  | 174.47 s  | 23.63 s    | 908.88 ms |
| s-k-150-250  | inf       | 20838.12 s | 150.30 s  |
| Total        | inf       | 20871.16 s | 157.89 s  |

_Note: The "Total" row represents the cumulative runtime across all test cases._

The C program is unable to solve the s-k-150-250 testcase in a reasonable amount
of time.
