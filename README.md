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
Why would you not tell me every time it loops?! Everything becomes so much easier if that can be assumed... $O(N^2)$

## Day 9
#### Part 1 & 2
I think I'm getting used to Rust :) I was able to write most of the code without looking anything up. $O(N^2)$

## Day 10
#### Part 1
BFS works fine. Learned to use `VecDeque`.

#### Part 2
Rough. I ended up doubling the grid and then using `UnionFind`. I know the implementation can be made better but I didn't have time today.

## Day 11
#### Part 1 & Part 2
Again, storing the map as `Vec<Vec<char>>` instead of `Vec<String>` makes things a lot easier. I wish there was an `abs` function that takes two `usize`s because it's a lot of work to do what in C++ is just `abs(a - b)`.

## Day 12
#### Part 1
Since we don't have time constraints (and I didn't know what the part 2 was going to look like), I decided to look for all of the possibilities and count the ones that matched the conditions (practicing writing bitwise exhaustive search in Rust). Took longer than I expected but it works :P

#### Part 2
The problem stays the same but with harder constraints. We can't possibly exhaust $O(2^{100})$ so we will have to DP. Normal 3D DP worked just fine.

## Day 13
#### Part 1 & 2
Not much that's new. Though there is still room for optimization, the input is small enough to check everything multiple times. 

## Day 14
#### Part 1
Simulation.

#### Part 2
The difficult part is starting to become not Rust but the problem. Within a billion cycles it's probably going to repeat itself (I don't have proof), so I coded based on that assumption.