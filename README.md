# Self-playing Guessing Game

This example runs the guessing game binary from the Rust Book tutorial and then plays it by incrementing its guesses.
Communication between the two processes is asynchronous and managed with `tokio`.

To run the guessing game alone:

```shell
cargo run --bin game
```

To run the guessing game player:

```shell
cargo run --bin player
```
