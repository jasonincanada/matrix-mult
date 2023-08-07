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
