// This is a Rust implementation of the vector-scalar multiplication algorithm from the paper
// "Matrix Multiplication Using Only Addition" by D. Cussen/J. Ullman [1]. It uses actual
// multiplication in the base case but this would be swapped with something like the Russian
// Peasants algorithm if built into a real chip, for a fully addition-only algorithm
//
// [1] D. Cussen and J. Ullman. Matrix Multiplication Using Only Addition
//     https://doi.org/10.48550/arXiv.2307.01415

fn main()
{
    let result = scalar_mult(5, vec![3,1,4,1,5,9]);

    // [15, 5, 20, 5, 25, 45]
    println!("{:?}", result);
}

// This is algorithm 2.2 Vector-Scalar Multiplication from [1], but instead of a pointer
// vector P it uses a Vec of (i32, Vec<usize>) to remember which positions (usize) to write the
// final elements (i32) back to
fn scalar_mult(c     : i32,
               vector: Vec<i32>) -> Vec<i32>
{
    // add an index to remember the elements' original locations and to get the
    // vector into the right shape for go(c, _)
    let indexed: Vec<(usize, i32)> =
        vector.into_iter()
              .enumerate()
              .collect();

    go(c, indexed)
        .into_iter()
        .map(|(_, elem)| elem)
        .collect()
}

fn go(    c     : i32,
      mut vector: Vec<(usize,i32)>) -> Vec<(usize,i32)>
{
    // base case, one element left in the vector
    if vector.len() == 1
    {
        // TODO: replace with something like russian peasants algo for a true addition-only
        //       implementation on a chip, but a normal multiplication here will do
        let (_, elem) = vector[0];
        vec![ (0, c*elem) ]
    }
    else
    {
        let len = vector.len();

        // the step numbers below match the paper on page 3

        // 1. Sort: sort by the element only, the order we store the pointers in doesn't matter
        //          since they are all random-access writes in step 5
        vector.sort_by(|(_,e1), (_,e2)| e1.cmp(e2));

        // build a map from each element to a list of places it occurred in the vector
        let pointers: Vec<(i32, Vec<usize>)> = group_indices_by_elem(vector);

        // 2. Differences: build the differences vector D
        let elems = pointers.iter().map(|(elem,_)| *elem);
        let diffs: Vec<i32> = take_diffs(elems);

        // 3. Recursion: the recursive step, return D'
        let recursed: Vec<i32> = scalar_mult(c, diffs);

        // 4. Accumulate: build the vector S' (scanl1 (+) recursed)
        let cs: Vec<i32> = accumulate(recursed);

        // 5. Follow pointers: populate the final, scaled vector V' from elements of S'
        //    situating them according to the original pointer map we built
        let mut scaled: Vec<(usize,i32)> =
            (0..len as i32)
                .enumerate()
                .collect();

        for (k, (_, ps)) in pointers.into_iter().enumerate() {
            for p in ps {
                let (i, _) = scaled[p];
                scaled[p] = (i, cs[k]);
            }
        }

        scaled
    }
}

// https://chat.openai.com/share/794ee6d1-868c-4417-bb31-c9bce2907273
fn group_indices_by_elem(indexed: Vec<(usize,i32)>) -> Vec<(i32,Vec<usize>)>
{
    let mut result: Vec<(i32,Vec<usize>)> = vec![];
    for (i, elem) in indexed {
        match result.last_mut() {
            Some((el, is)) if *el == elem => is.push(i),
            _                             => result.push((elem, vec![i])),
        }
    }
    result
}

// copy the first element of an i32 iterator then take the diffs
// between subsequent pairs of elements
fn take_diffs<I>(mut iter: I) -> Vec<i32>
where
    I: Iterator<Item=i32>
{
    let mut diffs: Vec<i32> = Vec::new();

    if let Some(first) = iter.next() {
        // copy the first element
        diffs.push(first);

        // take diffs of the rest
        let mut prev = first;
        for num in iter {
            diffs.push(num - prev);
            prev = num;
        }
    }
    diffs
}

// https://chat.openai.com/share/a383c128-503f-476c-a8b7-883f92e4bb5d
fn accumulate(vector: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    vector.iter()
          .scan(0, |state, &x| {
                       *state += x;
                       Some(*state)
                   })
          .for_each(|x| result.push(x));

    result
}

// TODO: "In practice, we would compute a complete outer product by
// multiplying V by n different constants, of which 5 was just
// an example. We use the same D and P vectors for each of
// the n constants, so there is no duplication of work."
//
// [1] p. 4


/* Tests */

#[cfg(test)]
mod tests {
    use super::{scalar_mult, group_indices_by_elem, take_diffs, accumulate};

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
    fn test_group() {
        assert_eq!(vec![(1, vec![1,3]),
                        (3, vec![0]),
                        (4, vec![2]),
                        (5, vec![4]),
                        (9, vec![5]),],
                    group_indices_by_elem(vec![(1,1), (3,1), (0,3), (2,4), (4,5), (5,9)]));
    }

    #[test]
    fn test_diff_vec_normal() {
        let v = vec![1, 2, 4, 7, 11, 16];
        let diff = take_diffs(v.into_iter());
        assert_eq!(diff, vec![1, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_diff_vec_empty() {
        let v: Vec<i32> = Vec::new();
        let diff = take_diffs(v.into_iter());
        assert_eq!(diff, Vec::new());
    }

    #[test]
    fn test_diff_vec_single_element() {
        let v = vec![10];
        let diff = take_diffs(v.into_iter());
        assert_eq!(diff, vec![10]);
    }

    #[test]
    fn test_diff_vec_negatives() {
        let v = vec![5, -3, -8, 1];
        let diff = take_diffs(v.into_iter());
        assert_eq!(diff, vec![5, -8, -5, 9]);
    }

    #[test]
    fn test_scanl() {
        assert_eq!(accumulate(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(accumulate(vec![1, 1, 1, 1]), vec![1, 2, 3, 4]);
        assert_eq!(accumulate(vec![3, -2, 5, -1]), vec![3, 1, 6, 5]);
        assert_eq!(accumulate(vec![]), vec![]);
    }
}
