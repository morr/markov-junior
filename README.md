## how to run
```sh
cargo build --release && time cargo run --release -- --size 150 --output output.txt && cat output.txt | pattern-to-png 1x | imgcat --width=50
cargo build --release && time cargo run --release -- --size 150 --output output.txt --seed 14440708802582084752 && cat output.txt | pattern-to-png 1x | imgcat --width=50
```

## profile
```sh
flamegraph -o /tmp/flame.svg -F 4999 -- target/debug/markov_junior
```
