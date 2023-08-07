
```rust
// https://chat.openai.com/share/43a45a33-ddd4-4ce1-92d8-db18f3df4574
fn take_diffs(v: Vec<&i32>) -> Vec<i32> {
    let mut diff: Vec<i32> = Vec::new();
    for i in 0..v.len() {
        if i == 0 {
            diff.push(v[i].clone());
        } else {
            diff.push(v[i] - v[i-1]);
        }
    }
    diff
}

// https://chat.openai.com/share/a383c128-503f-476c-a8b7-883f92e4bb5d
fn accumulate<I>(numbers: I) -> Vec<i32>
where
    I: Iterator<Item=i32>
{
    let mut result: Vec<i32> = Vec::new();
    numbers.scan(0, |state, el| {
                        *state += el;
                        Some(*state)
                    })
           .for_each(|x| result.push(x));
    result
}

// This is algorithm 2.2 Vector-Scalar Multiplication from [1], but instead of a pointer
// vector P it uses a Vec of (i32, Vec<usize>) to remember which positions (usize) to write the
// final elements (i32) back to
fn scalar_mult(c     : i32,
               vector: Vec<i32>) -> Vec<i32>
{
    // add an index to remember the elements' original locations and to get the
    // vector into the right shape for go(c, _)
    let indexed: Vec<(usize, (i32,u32))> =
        vector.into_iter()
              .enumerate()
              // align each element by shifting off all zero bits on the right
              .map(|(i, elem)| (i, align(elem)))
              .collect();

    go(c, indexed)
        .into_iter()
        .map(|(_, (elem, _))| elem)
        .collect()
}

fn go(    c     : i32,
      mut vector: Vec<(usize,(i32,u32))>) -> Vec<(usize,(i32,u32))>
{
    // base case, one element left in the vector
    if vector.len() == 1
    {
        // TODO: replace with something like russian peasants algo for a true addition-only
        //       implementation on a chip, but a normal multiplication here will do
        let (_, (elem, shift)) = vector[0];
        vec![ (0, (c*elem, shift)) ]
    }
    else
    {
        let len = vector.len();

        // the step numbers below match the paper on page 3

        // 1. Sort: sort by the element only; within an element group the order we store the
        //          pointers doesn't matter since they are all random-access writes in step 5
        vector.sort_by(|(_,(e1,_)), (_,(e2,_))| e1.cmp(e2));

        // build a map from each distinct element to a list of places it occurred in the vector
        let pointers: Vec<(i32, Vec<(usize,u32)>)> = group_indices_by_elem(vector);

        // 2. Differences: build the differences vector D
        let elems = pointers.iter().map(|(elem,_)| *elem);
        let diffs: Vec<(usize,(i32,u32))> =
            take_diffs(elems)
                .enumerate()
                .map(|(i, elem)| (i, align(elem)))
                .collect();

        // 3. Recursion: the recursive step, return D'
        let recursed: Vec<(usize,(i32,u32))> = go(c, diffs);

        // 4. Accumulate: build the vector S' (scanl1 (+) recursed)
        let elems = recursed.into_iter().map(|(_,(elem,_))| elem);
        let cs: Vec<i32> = accumulate(elems).collect();

        // 5. Follow pointers: populate the final, scaled vector V' from elements of S'
        //    situating them according to the original pointer map we built
        let mut scaled: Vec<(usize,(i32,u32))> = vec![ (0,(0,0)); len ];

        for (k, (_, ps)) in pointers.into_iter().enumerate() {
            for (p, shift) in ps {
                scaled[p] = (p, (cs[k] << shift, 0)); // TODO: this 0 can be anything and the result is still correct??
            }
        }

        scaled
    }
}

fn scalar_mult(c     : i32,
               vector: Vec<i32>) -> Vec<i32>
{
    let steps: Vec<StepState> = vec![];

    // the top half of Figure 1, drill down to the base case, collecting information
    // about the transformations along the way in the steps vector
    let (mut steps, last_element) = down(vector, steps);

    steps.reverse();

    // the bottom half of Figure 1
    up(&steps, vec![last_element*c])
}
```

```rust
/* Accumulate iterator */

// wrap an iterator of integers and return the running total as a new iterator
struct Accumulate<I: Iterator<Item=i32>> {
    iter: I,   // the underlying iterator of i32s
    sum : i32, // keep track of the sum between calls to next()
}

impl<I> Iterator for Accumulate<I>
where
    I: Iterator<Item=i32>
{
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self.iter.next() {
            Some(int) => {
                self.sum += int;
                Some(self.sum)
            },
            None => None
        }
    }
}

fn accumulate<I>(iter: I) -> Accumulate<I>
where
    I: Iterator<Item=i32>
{
    Accumulate {
        iter,
        sum: 0
    }
}
```

```rust
// clippy helped with this
        if c == 0 {
            zeros.push(i);
        } else if c < 0 {
            negatives.push(i);
            ints.push(-c);
        } else {
            ints.push(c);
        }
```

```rust
#[cfg(test)]
mod tests {
    use super::scalar_mult;

    #[test]
    fn test() {
        // check 5*[3,1,4,1,5,9] = [15,5,20,5,15,45]
        assert_eq!(vec![5*3,
                        5*1,
                        5*4,
                        5*1,
                        5*5,
                        5*9], scalar_mult(5, vec![3,1,4,1,5,9]));
    }

    #[test]
    fn test_scanl() {
        assert_eq!(accumulate(vec![1, 2, 3, 4].into_iter()).collect::<Vec<_>>(), vec![1, 3, 6, 10]);
        assert_eq!(accumulate(vec![1, 1, 1, 1].into_iter()).collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert_eq!(accumulate(vec![3, -2, 5, -1].into_iter()).collect::<Vec<_>>(), vec![3, 1, 6, 5]);
        assert_eq!(accumulate(vec![].into_iter()).collect::<Vec<_>>(), vec![]);
    }
}
```
