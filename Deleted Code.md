
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
```
