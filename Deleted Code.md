
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
```
