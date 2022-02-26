# About
This repo demonstrates unusually long compile times when building with
optimizations due to the optimizer trying to optimize
`serde_json::from_str::<BigStruct>(include_str!("big_json.json"))`

This bug exists in stable rustc 1.59.0

Build with optimizations to see the bug, code to generate the struct and json
are included so you can add more or fewer fields easily.

```
$ cargo build --release
```