## how to run
```sh
cargo run | pattern-to-png 10x | imgcat
```

```sh
 cargo run --release | pattern-to-png 1x | imgcat --width=150 --height 50
```

## profile
```sh
flamegraph -o /tmp/flame.svg -F 4999 -- target/debug/markov_junior
```
