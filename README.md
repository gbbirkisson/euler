<h1>
  <p align="center">
    <a href="https://github.com/gbbirkisson/euler">
      <img src="https://projecteuler.net/images/clipart/euler_portrait.png" alt="Logo" height="128">
    </a>
    <br>Project Euler
  </p>
</h1>

In this repository is my humble attempt to solve some [Project Euler problems](https://projecteuler.net/archives)
using Rust. This is just for fun and to learn the Rust programming language.

Any solution to a problem should not take more than one minute to solve! This is a quote from their website:

> Each problem has been designed according to a "one-minute rule", which means that although it may take several hours to design a successful algorithm with more difficult problems, an efficient implementation will allow a solution to be obtained on a modestly powered computer in less than one minute.

## Running tests

```console
$ cargo test --release -- --test-threads=4 --show-output
```

## Running a single solution

For example, if you want to run the solution to problem `4` run:

```console
$ cargo run --release -- -p 4
```

## Test + Linting

```console
$ make
```

## Problems

> **Note**: Solutions that take more than 1 second are marked with: 🐢

- [x] [001](src/problems/p001.rs) Multiples of 3 and 5
- [x] [002](src/problems/p002.rs) Even Fibonacci numbers
- [x] [003](src/problems/p003.rs) Largest prime factor
- [x] [004](src/problems/p004.rs) Largest palindrome product
- [x] [005](src/problems/p005.rs) Smallest multiple
- [x] [006](src/problems/p006.rs) Sum square difference
- [x] [007](src/problems/p007.rs) 10001st prime
- [x] [008](src/problems/p008.rs) Largest product in a series
- [x] [009](src/problems/p009.rs) Special Pythagorean triplet
- [x] [010](src/problems/p010.rs) Summation of primes
- [x] [011](src/problems/p011.rs) Largest product in a grid
- [x] [012](src/problems/p012.rs) Highly divisible triangular number
- [x] [013](src/problems/p013.rs) Large sum
- [x] [014](src/problems/p014.rs) Longest Collatz sequence
- [x] [015](src/problems/p015.rs) Lattice paths
- [x] [016](src/problems/p016.rs) Power digit sum
- [x] [017](src/problems/p017.rs) Number letter counts
- [x] [018](src/problems/p018.rs) Maximum path sum I
- [x] [019](src/problems/p019.rs) Counting Sundays
- [x] [020](src/problems/p020.rs) Factorial digit sum
- [x] [021](src/problems/p021.rs) Amicable numbers 🐢
- [ ] 022 Names scores
- [ ] 023 Non-abundant sums
- [ ] 024 Lexicographic permutations
- [ ] 025 1000-digit Fibonacci number
- [ ] 026 Reciprocal cycles
- [ ] 027 Quadratic primes
- [ ] 028 Number spiral diagonals
- [ ] 029 Distinct powers
- [ ] 030 Digit fifth powers
- [ ] 031 Coin sums
- [ ] 032 Pandigital products
- [ ] 033 Digit cancelling fractions
- [ ] 034 Digit factorials
- [ ] 035 Circular primes
- [ ] 036 Double-base palindromes
- [ ] 037 Truncatable primes
- [ ] 038 Pandigital multiples
- [ ] 039 Integer right triangles
- [ ] 040 Champernowne's constant
- [ ] 041 Pandigital prime
- [ ] 042 Coded triangle numbers
- [ ] 043 Sub-string divisibility
- [ ] 044 Pentagon numbers
- [ ] 045 Triangular, pentagonal, and hexagonal
- [ ] 046 Goldbach's other conjecture
- [ ] 047 Distinct primes factors
- [ ] 048 Self powers
- [ ] 049 Prime permutations
- [ ] 050 Consecutive prime sum
- [ ] 051 Prime digit replacements
- [ ] 052 Permuted multiples
- [ ] 053 Combinatoric selections
- [ ] 054 Poker hands
- [ ] 055 Lychrel numbers
- [ ] 056 Powerful digit sum
- [ ] 057 Square root convergents
- [ ] 058 Spiral primes
- [ ] 059 XOR decryption
- [ ] 060 Prime pair sets
- [ ] 061 Cyclical figurate numbers
- [ ] 062 Cubic permutations
- [ ] 063 Powerful digit counts
- [ ] 064 Odd period square roots
- [ ] 065 Convergents of e
- [ ] 066 Diophantine equation
- [x] [067](src/problems/p067.rs) Maximum path sum II
- [ ] 068 Magic 5-gon ring
- [ ] 069 Totient maximum
- [ ] 070 Totient permutation
- [ ] 071 Ordered fractions
- [ ] 072 Counting fractions
- [ ] 073 Counting fractions in a range
- [ ] 074 Digit factorial chains
- [ ] 075 Singular integer right triangles
- [ ] 076 Counting summations
- [ ] 077 Prime summations
- [ ] 078 Coin partitions
- [ ] 079 Passcode derivation
- [ ] 080 Square root digital expansion
- [ ] 081 Path sum: two ways
- [ ] 082 Path sum: three ways
- [ ] 083 Path sum: four ways
- [ ] 084 Monopoly odds
- [ ] 085 Counting rectangles
- [ ] 086 Cuboid route
- [ ] 087 Prime power triples
- [ ] 088 Product-sum numbers
- [ ] 089 Roman numerals
- [ ] 090 Cube digit pairs
- [ ] 091 Right triangles with integer coordinates
- [ ] 092 Square digit chains
- [ ] 093 Arithmetic expressions
- [ ] 094 Almost equilateral triangles
- [ ] 095 Amicable chains
- [ ] 096 Su Doku
- [ ] 097 Large non-Mersenne prime
- [ ] 098 Anagramic squares
- [ ] 099 Largest exponential
- [ ] 100 Arranged probability
