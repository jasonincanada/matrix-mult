## 24 July 2023

While testing `outer_product()`, I found that the algorithm doesn't terminate if there's a 0 in a row. After a row vector with both zeroes and non-zeroes is sorted and de-duplicated, the vector and the resulting `diffs` vector will have a 0 in front and a non-zero value beside it. The `down()` function will end up calling the two-element `[0,x]` vector (non-zero x) repeatedly, so it never enters the base case where `vector.len() == 1`
