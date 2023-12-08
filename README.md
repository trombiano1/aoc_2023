My attempts at solving Advent of Code 2023.

I thought this was a great opportunity to learn to write in Rust so this is my first Rust project. I was suffering at first but I'm getting there :)


# Diary
## Day 1
#### Part 1
My first Rust code! 

The logic is simple ($O(N\times L)$ with $N$ being the input count and $L$ being the max input length) but I'm struggling with getting the input - apparently a crate called `proconio` is useful for competitive-programming-style-input but since we don't get the number of lines here I think we need to iterate?

I also need to get used to `Optional` (`unwrap`, `Some`).

#### Part 2
Getting nth char of `String` seems to be of much more trouble than an array of `char`; not sure which I should be using for this style of coding.

$O(N \times L \times 9 \times l)$ with $l$ being the longest spelled-out number (`three` with 5 chars).

## Day 2
#### Part 1
Tried using `HashMap` though it's not really necessary here. The logic is simple enough. $O(N)$ with $N$ being the number of total games.

#### Part 2
Felt part 2 was easier than part 1. $O(N)$.

## Day 3
#### Part 1
A bit harder than the first two days - a nice challenge for learning a new programming language. I feel storing things in `Char` made things a bit easier. Also learned how to write functions.

#### Part 2
Part 2 wasn't all that different from part 1 (except it required the use of `HashMap`).

## Day 4
#### Part 1
Tried using `HashSet`; it works pretty much the same as in C++.

#### Part 2
Almost the same as part 1, the only difference being the time complexity. The input is small enough so I ignored.

## Day 5
#### Part 1
Naive method works just fine. Tuples are so good in Rust - I wish C++ had this kind of syntax instead of `get<n>(t)` nonsense.

#### Part 2
This is the first time I struggled a bit with logic. I realized I can just brute force all of the `seeds` as we don't have runtime constraints. Compiling with `-r` made the code run _A LOT_ quicker; wish I knew this before tackling this problem! 

## Day 6
#### Part 1 & Part 2
Highschool level algebra.

## Day 7
#### Part 1
Getting used to using `match` and closures, for simple functions and also for sorting. I feel this is the first time I actually benefitted from using Rust. Maybe enums would have been of good use here.

#### Part 2
Almost the same as part 1.

## Day 8
#### Part 1
Naive method is just fine, simple enough.

#### Part 2
Why would you not tell me every time it loops?! Everything becomes so much easier if that can be assumed...($O(N^2)$)