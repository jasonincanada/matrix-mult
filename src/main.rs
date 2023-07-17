// This is a Rust implementation of the matrix multiplication algorithm from the paper
// "Matrix Multiplication Using Only Addition" by D. Cussen/J. Ullman [1]. It uses actual
// multiplication in the base case but this would be swapped with something like the Russian
// Peasants algorithm if built into a real chip, for a fully addition-only algorithm
//
// [1] D. Cussen, J. Ullman. "Matrix Multiplication Using Only Addition."
//     arXiv preprint arXiv:2307.01415 (2023).
//     https://doi.org/10.48550/arXiv.2307.01415

fn main()
{
    // demo outer product
    let col     = vec![0,1,2,3,4,5];
    let row     = vec![3,1,4,1,5,9];
    let product = outer_product(&col, &row);
    println!("outer_product(col, row) = {:?}", product);

    // demo full matrix multiplication
    let a = Matrix { rows: 2, cols: 3, elems: vec![ vec![1,2,3],
                                                    vec![4,5,6] ]};
    let b = Matrix { rows: 3, cols: 2, elems: vec![ vec![7,8],
                                                    vec![9,10],
                                                    vec![11,12] ]};
    let matrix = matrix_mult(a, b);
    println!("matrix_mult(a, b) = {:?}", matrix);
}

// multiply an m-by-k matrix by a k-by-n matrix using the algo in the paper, to give an m-by-n matrix
fn matrix_mult(a: Matrix<i32>,
               b: Matrix<i32>) -> Matrix<i32>
{
    assert_eq!(a.cols, b.rows);

    // transpose a to make it easier to refer to columns from it (as rows)
    let a_t = a.transpose();

    // add the k outer products together to construct the resulting matrix
    let mut result = zeros(a.rows, b.cols);
    for k in 0 .. a.cols {
        let col    : &Vec<i32>   = &a_t.elems[k];
        let row    : &Vec<i32>   = &b  .elems[k];
        let product: Matrix<i32> = outer_product(col, row);

        result += product;
    }

    result
}

// construct the m-by-n matrix generated by the outer product of a m-element column vector
// with an n-element row vector
fn outer_product(col: &Vec<i32>,
                 row: &Vec<i32>) -> Matrix<i32>
{
    let steps: Vec<StepState> = vec![];

    // the top half of Figure 1: recursively drill down() to the base case,
    // remembering the transformations along the way in the steps vector
    let (last_element, mut steps) = down(row.clone(), steps);
    steps.reverse();

    // we can now reuse the information in steps for each element of the column vector.
    // in fact, since steps won't change for the rest of this outer product calculation,
    // let's rebind it as non-mutable
    let steps = steps;

    let mut rows: Vec<Vec<i32>> = vec![];

    for c in col {
        // the bottom half of Figure 1: start with the element left over from the recursive down() phase,
        // multiply it by this column element c, then recursively expand it back to its full row width
        let row = up(&steps, vec![c*last_element]);
        rows.push(row);
    }

    Matrix {
        elems: rows,
        rows : col.len(),
        cols : row.len()
    }
}

// the top half of Figure 1. this very quickly reduces the vector down to a single element,
// while keeping track of the restructuring it does along the way, so we can carry out the
// reverse operations in the up() phase
fn down(    vector: Vec<i32>,
        mut steps : Vec<StepState>) -> (i32, Vec<StepState>)
{
    assert!(!vector.is_empty());

    if vector.len() == 1 {
        return (vector[0], steps)
    }

    // use enumerate() to pair up each element with its location (usize) in the vector,
    // then call align on the element, which shifts off the rightmost zero bits, keeping
    // track of the resulting integer (i32) and the number of zero bits shifted off (u32)
    let mut v: Vec<(usize, (i32,u32))> =
        vector.iter()
              .enumerate()
              .map(|(i, elem)| (i, align(*elem)))
              .collect();

    // the step numbers below match the paper on page 3

    // 1. Sort: sort by the element only; within an element group the order we store the
    //          pointers doesn't matter since they are all random-access writes in step 5
    v.sort_by(|(_,(e1,_)), (_,(e2,_))| e1.cmp(e2));

    // build a map from each distinct element to a list of places it occurred in the vector,
    // and how many bits were shifted off (maybe 0) at each location
    let pointers: PointersAndShifts = group_indices_by_elem(v);

    // 2. Differences: build the differences vector D
    let elems = pointers.iter().map(|(elem,_)| *elem);
    let diffs: Vec<i32> = take_diffs(elems).collect();

    steps.push(StepState {
                   len: vector.len(),
                   pointers
               });

    down(diffs, steps)
}

fn up(    steps: &[StepState],
      mut vec  : Vec<i32>) -> Vec<i32>
{
    if steps.is_empty() {
        return vec
    }

    // 4. Accumulate: build the vector S' in place
    accumulate(&mut vec);

    // 5. Follow Pointers: populate the final, scaled vector V' from elements of S'
    //    situating and unshifting them according to the original pointer map we built
    let mut scaled: Vec<i32> = vec![ 0; steps[0].len ];

    for (k, (_, ps)) in steps[0].pointers.iter().enumerate() {
        for (p, shift) in ps {
            scaled[*p] = vec[k] << shift;
        }
    }

    // recurse with the next step and the vector transformed up to this point
    up(&steps[1..], scaled)
}

// map of distinct elements to their locations (usize) in the vector and the number (u32) of zero bits
// that were shifted off to the right to divide out powers of two
type PointersAndShifts = Vec<(i32, Vec<(usize,u32)>)>;

// each call to down() except the final one generates a StepState record to track what it did
struct StepState
{
    // length of the vector at the start of the step
    len: usize,

    // record of the operations that sorted, de-duplicated, and right-shifted the elements of the vector
    // in the down() phase, so we can reverse them in the up() phase when generating the outer products
    pointers: PointersAndShifts
}

// do a scanl1 (+) in-place mutably
fn accumulate(vec: &mut Vec<i32>) {
    for i in 1 .. vec.len() {
        vec[i] += vec[i-1];
    }
}

// shift off the rightmost zeros and remember how many there were
// https://chat.openai.com/share/a4c49643-8b14-44bb-a8e6-3b81bfe10e0c
fn align(elem: i32) -> (i32, u32) {
    if elem == 0 {
        (elem, 0)
    } else {
        let shifts = elem.trailing_zeros();
        (elem >> shifts, shifts)
    }
}

// https://chat.openai.com/share/794ee6d1-868c-4417-bb31-c9bce2907273
fn group_indices_by_elem(indexed: Vec<(usize,(i32,u32))>) -> Vec<(i32,Vec<(usize,u32)>)>
{
    let mut result: Vec<(i32,Vec<(usize,u32)>)> = vec![];
    for (i, (elem,shift)) in indexed {
        match result.last_mut() {
            Some((el, is)) if *el == elem => is.push((i,shift)),
            _                             => result.push((elem, vec![(i,shift)])),
        }
    }
    result
}


/* m-by-n Matrices */

#[derive(Debug)]
struct Matrix<T> {
    elems: Vec<Vec<T>>,
    rows : usize,
    cols : usize,
}

// chatgpt 4.0
// this lets us compare two matrices with == for unit tests
impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }

        for i in 0..self.rows {
        for j in 0..self.cols {
            if self.elems[i][j] != other.elems[i][j] {
                return false;
            }
        }}

        true
    }
}

// this lets us use += in the line "result += product" to accumulate the outer products
impl AddAssign for Matrix<i32> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        for i in 0..self.rows {
        for j in 0..self.cols {
            self.elems[i][j] += rhs.elems[i][j];
        }}
    }
}

// matrix transposition from chatgpt 4.0
impl<T: Copy + Default> Matrix<T> {
    fn transpose(&self) -> Self {
        let mut result = Matrix {
            elems: vec![vec![T::default(); self.rows]; self.cols],
            rows : self.cols,
            cols : self.rows,
        };

        for i in 0..self.rows {
        for j in 0..self.cols {
            result.elems[j][i] = self.elems[i][j];
        }}

        result
    }
}

// construct a rows-by-cols matrix with all zeros
fn zeros(rows: usize,
         cols: usize) -> Matrix<i32>
{
    Matrix {
        elems: vec![ vec![0; cols]; rows ],
        rows,
        cols
    }
}


/* TakeDiffs iterator */

// wrap an iterator of integers and return the first one as-is, then differences
// between the subsequent pairs of integers
struct TakeDiffs<I: Iterator<Item=i32>> {
    iter    : I,           // the underlying iterator of i32s
    previous: Option<i32>, // remember the last number we saw
}

impl<I> Iterator for TakeDiffs<I>
where
    I: Iterator<Item=i32>
{
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self.iter.next() {
            Some(int) => {
                match self.previous {
                    Some(prev) => { self.previous = Some(int); Some(int - prev) },
                    None       => { self.previous = Some(int); Some(int)        }
                }
            },
            None => None
        }
    }
}

fn take_diffs<I>(iter: I) -> TakeDiffs<I>
where
    I: Iterator<Item=i32>
{
    TakeDiffs {
        iter,
        previous: None
    }
}


/* Imports */

use std::ops::AddAssign;


/* Tests */

#[cfg(test)]
mod tests {
    use super::{align, group_indices_by_elem, take_diffs, outer_product, matrix_mult, Matrix};

    #[test]
    fn test_group() {
        assert_eq!(vec![(1, vec![(1,0),(3,0)]),
                        (3, vec![(0,0)]),
                        (4, vec![(2,0)]),
                        (5, vec![(4,0)]),
                        (9, vec![(5,0)]),],
                    group_indices_by_elem(vec![(1,(1,0)), (3,(1,0)), (0,(3,0)), (2,(4,0)), (4,(5,0)), (5,(9,0))]));
    }

    #[test]
    fn test_diff_vec_normal() {
        let v = vec![1, 2, 4, 7, 11, 16];
        let diff: Vec<i32> = take_diffs(v.into_iter()).collect();
        assert_eq!(diff, vec![1,1,2,3,4,5]);
    }

    #[test]
    fn test_diff_vec_empty() {
        let v: Vec<i32> = Vec::new();
        let diff: Vec<i32> = take_diffs(v.into_iter()).collect();
        assert_eq!(diff, Vec::new());
    }

    #[test]
    fn test_diff_vec_single_element() {
        let v = vec![10];
        let diff: Vec<i32> = take_diffs(v.into_iter()).collect();
        assert_eq!(diff, vec![10]);
    }

    #[test]
    fn test_diff_vec_negatives() {
        let v = vec![5, -3, -8, 1];
        let diff: Vec<i32> = take_diffs(v.into_iter()).collect();
        assert_eq!(diff, vec![5,-8,-5,9]);
    }

    /* align(), three tests from chatgpt 4.0 */
    #[test]
    fn test_align_zero() {
        let (aligned_elem, shifts) = align(0);
        assert_eq!(aligned_elem, 0);
        assert_eq!(shifts, 0);
    }

    #[test]
    fn test_align_no_trailing_zeros() {
        let (aligned_elem, shifts) = align(7); // 7 is 111 in binary
        assert_eq!(aligned_elem, 7);
        assert_eq!(shifts, 0);
    }

    #[test]
    fn test_align_trailing_zeros() {
        let (aligned_elem, shifts) = align(16); // 16 is 10000 in binary
        assert_eq!(aligned_elem, 1);
        assert_eq!(shifts, 4);
    }

    // chatgpt 4.0
    #[test]
    fn test_outer_product_same_length() {
        let col = vec![1, 2, 3];
        let row = vec![4, 5, 6];
        let result = outer_product(&col, &row);

        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 3);

        let expected_grid = vec![
            vec![4, 5, 6],
            vec![8, 10, 12],
            vec![12, 15, 18],
        ];
        
        assert_eq!(result.elems, expected_grid);
    }

    // chatgpt 4.0
    #[test]
    fn test_matrix_mult_normal() {
        let a = Matrix {
            elems: vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ],
            rows: 2,
            cols: 3,
        };
        let b = Matrix {
            elems: vec![
                vec![7, 8],
                vec![9, 10],
                vec![11, 12]
            ],
            rows: 3,
            cols: 2,
        };
        let result = matrix_mult(a, b);

        let expected = Matrix {
            elems: vec![
                //vec![58, 64],     [jrh] i'm surprised chatgpt got these numbers right;
                //vec![139, 154],         they probably weren't actually calculated

                // [jrh] expand to see the calculations
                vec![1*7 + 2*9 + 3*11,  1*8 + 2*10 + 3*12],
                vec![4*7 + 5*9 + 6*11,  4*8 + 5*10 + 6*12],
            ],
            rows: 2,
            cols: 2,
        };

        assert_eq!(result, expected);
    }
}
