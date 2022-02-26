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


Quick and dirty benchmark of `cargo build --release` from a clean state on my machine, ryzen 3900x
```
Size | 1.58.1 | 1.59.0
----------------------
50   |     8s |    10s
100  |    12s |    36s
200  |    31s |   427s
300  |    83s | >1800s
```