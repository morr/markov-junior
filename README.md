## how to run
```sh
cargo build --release && time cargo run --release -- -o output.txt && cat output.txt | pattern-to-png 1x | imgcat --width=50 --height 25
```

## profile
```sh
flamegraph -o /tmp/flame.svg -F 4999 -- target/debug/markov_junior
```
