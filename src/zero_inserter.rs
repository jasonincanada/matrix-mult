
/* ZeroInserter Iterator */

struct ZeroInserter<I: Iterator<Item=i32>> {
    iter     : I,           // the underlying iterator of i32s
    iter_idx : usize,
    zeros    : Vec<usize>,  // insert the zeros at these indexes
    zeros_idx: usize,
}


// assumptions:
// - ZeroInserter.zeros is sorted in ascending order
// - each zero index doesn't exceed the length of the final vector
impl<I> Iterator for ZeroInserter<I>
where
    I: Iterator<Item=i32>
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.zeros_idx == self.zeros.len() {
            return self.iter.next()
        }
        
        // does the next zero go at this index
        if self.iter_idx == self.zeros[self.zeros_idx] {
            self.iter_idx  += 1;
            self.zeros_idx += 1;
            return Some(0)
        }

        self.iter_idx += 1;
        self.iter.next()
    }
}

fn zero_inserter<I>(iter: I, zeros: Vec<usize>) -> ZeroInserter<I>
where
    I: Iterator<Item=i32>
{
    ZeroInserter {
        iter,
        iter_idx: 0,
        zeros,
        zeros_idx: 0
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::zero_inserter;

    #[test]
    fn test_zero_inserter_empty_lists() {
        let row   = vec![];
        let zeros = vec![];
        assert_eq!(Vec::<i32>::new(), zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }

    #[test]
    fn test_zero_inserter_singleton_row() {
        let row   = vec![1];
        let zeros = vec![];
        assert_eq!(vec![1], zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }

    #[test]
    fn test_zero_inserter_singleton_zero() {
        let row   = vec![];
        let zeros = vec![0]; // 0:usize not 0:i32
        assert_eq!(vec![0], zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }

    #[test]
    fn test_zero_inserter_many_row_zero_first() {
        let row   = vec![1,2,3];
        let zeros = vec![0];
        assert_eq!(vec![0,1,2,3], zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }

    #[test]
    fn test_zero_inserter_many_row_zero_second() {
        let row   = vec![1,2,3];
        let zeros = vec![1];
        assert_eq!(vec![1,0,2,3], zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }

    #[test]
    fn test_zero_inserter_many_row_zero_third() {
        let row   = vec![1,2,3];
        let zeros = vec![2];
        assert_eq!(vec![1,2,0,3], zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }

    #[test]
    fn test_zero_inserter_many_row_trailing_zeros() {
        let row   = vec![1,2,3];
        let zeros = vec![3,4];
        assert_eq!(vec![1,2,3,0,0], zero_inserter(row.into_iter(), zeros).collect::<Vec<_>>());
    }
}
