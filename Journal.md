## 26 August 2023

I wrote an iterator-based ZeroInserter, though I'm keeping it in a different file and haven't actually used it, since it is much less readable than the current solution of directly inserting the zeros at specific indices (and accepting the quadratic running time from having to push all subsequent elements down by one index).

I've coded a few implementations of `Iterator` since I started using Rust last year. They are a nice tool for the task at hand, but I think Rust isn't the right language to express them. There's too much punctuation and syntactic clutter considering how simple the underlying idea is. It's possible that my use-cases have been too simple and only in sufficiently complex iterators does the substance start to outweigh the amount of code required to express it. But I think most iterators will be simple "value-add" steps in a broader pipeline of them, with minimal state to track for each, so they should be a lot more lightweight to compose than they currently are, in Rust anyway.

Iterators I've written so far:

**ZeroInserter** ([matrix-mult](https://github.com/jasonincanada/matrix-mult/blob/main/src/zero_inserter.rs)) - Insert zeros at specific indices in an underlying iterator of `i32`s

**IntervalMerger** ([aoc-2022/day15](https://github.com/jasonincanada/aoc-2022/blob/main/days/day_15/src/main.rs#L120)) - Merge a new interval at the right spot in the underlying iterator of intervals

**AngleIterator** ([kattis/toast](https://github.com/jasonincanada/kattis/blob/master/rust/toast/src/main.rs#L100)) - Produce the angles needed to check for n people sitting around a table

**Repeater** ([aoc-2022/day17](https://github.com/jasonincanada/aoc-2022/blob/main/days/day_17/src/main.rs#L235)) - Wrap an iterator and return its items in a cycle forever

**PairPrefixSwapper** ([kattis/spumbers2](https://github.com/jasonincanada/kattis/blob/master/rust/spumbers2/src/main.rs#L238)) - ??

**FactorPair** ([leetcode/prod-except-self](https://github.com/jasonincanada/kattis/blob/master/rust/prod-except-self/src/main.rs#L42)) - ??


## 17 August 2023

https://www.youtube.com/watch?v=algDLvbl1YY&t=611s - The Dark Side of .reserve()
https://github.com/facebook/folly/blob/main/folly/docs/FBVector.md#memory-handling


## 16 August 2023

- need a new language for iterators, Rust is suddenly too complicated or at least too much punctuation


## 14 August 2023

```haskell
go :: [Int] -> [Usize] -> [Int]
go ints []     = ints
go ints (u:us) = 
```


## 6 August 2023

### prepare()

It feels like this function should have a different name, but I don't know what it should be. It might turn out to be a useful pre-processing step for other algorithms as well. It takes a list of integers and returns three lists:

- `zeros`: locations of the zeros in the row (`Vec<usize>`)
- `negatives`: locations of the negative numbers in the row (`Vec<usize>`)
- `naturals`: absolute values of the numbers in the row, preserving their original order, except for the zeros, which have been removed (`Vec<i32>`)

Each list can be no longer than the original list, and the total length of the three of them can be at most twice the length of the original list. (Though for efficiency's sake, in the function a full row width is allocated ahead of time for each list, to avoid re-allocations by the `push()` calls)

This function is used in `outer_product()` to pre-process the row before passing it into the recursive steps. The refined list `naturals` is the initial row passed to `down()`, which now doesn't need to worry about the minimum integer being a zero. `down()/up()` should work fine if the row contains negative numbers, but the signs are removed before processing anyway to encourage duplication in the resulting list--the same reason we shift off right zero bits in `align()`.

```rust
fn prepare(row: &[i32]) -> (Vec<usize>, Vec<usize>, Vec<i32>) {
    let mut zeros    : Vec<usize> = Vec::with_capacity(row.len());
    let mut negatives: Vec<usize> = Vec::with_capacity(row.len());
    let mut naturals : Vec<i32>   = Vec::with_capacity(row.len());

    for (i, &c) in row.iter().enumerate() {
        match c.cmp(&0) {
            Greater => { naturals.push(c) },
            Less    => { naturals.push(-c); negatives.push(i); },
            Equal   => { zeros.push(i) }
        }
    }

    (zeros, negatives, naturals)
}
```

After the `up()` phase returns the list of naturals scaled by `c`, the final row vector is prepared by inserting the missing zeros at the right locations, then flipping signs back to negative at the right locations. The first loop is quadratic in time, so a more efficient zero-inserter could be written, but this more concisely conveys what's happening.

```rust
// add back in the zeros and negative signs
for i in &zeros {
    row.insert(*i, 0);
}
for i in &negatives {
    row[*i] *= -1
}
```

### Caching

There is an obvious caching step in `outer_product()` that I've left out for brevity. If we see the same column element `c` twice, we don't have to re-compute the row vector, since it will be the same as before (due to the functional purity of `down()/up()` and the immutability of `steps`). This data could be stored in a standard `HashMap<i32,Vec<i32>>` or `HashMap<i32,usize>` or similar structure.

### Signatures

```rust
fn prepare(row: &Vec<i32>) -> (Vec<usize>, Vec<usize>, Vec<i32>)
fn down(vector: Vec<i32>, mut steps: Vec<StepState>) -> (i32, Vec<StepState>)
fn up(steps: &[StepState], mut vec: Vec<i32>) -> Vec<i32>

type StepState = (usize, ReconstructionMap);
type ReconstructionMap = Vec<(i32, Vec<WhereAndShift>)>;
type WhereAndShift = (usize, u32);

type StepState = (usize, Vec<(i32,Vec<(usize,u32)>)>)
```

`StepState` doesn't need to keep the length (usize) of the vector, because it can be computed as the count of the usizes (alternatively their maximum plus 1) in the `Vec<usize,u32>` vectors, which is where the locations and shifts of the original values are stored. The length is kept around anyway to save the time cost of the traversal.


## 5 August 2023

There are hints of refactoring to be done here. In StepState, `has_zeros` is only true for the first step in the `down()` phase and then will never be true on subsequent steps. Similarly `is_negative` in an AlignedInt only matters at the last step in `up()`

```rust
type AlignedInt   = (i32  , ShiftAndSign);
type WhereAndHow  = (usize, ShiftAndSign);
type ShiftAndSign = (u32, bool);
```

Something is off about these types...

The shift we're going to need at every step. The sign only matters at the end, similarly with has_zeros. Should we pair up ShiftAndSign like that... 


## 2 August 2023

- the negative numbers are only in the first step, same with zeros. all subsequent steps don't have to store either of these data


## 24 July 2023

While testing `outer_product()`, I found that the algorithm doesn't terminate if there's a 0 in a row. After a row vector with both zeroes and non-zeroes is sorted and de-duplicated, the vector and the resulting `diffs` vector will have a 0 in front and a non-zero value beside it. The `down()` function will end up calling the two-element `[0,x]` vector (non-zero x) repeatedly, so it never enters the base case where `vector.len() == 1`
