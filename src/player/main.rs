mod game_state;

use game_state::GameState;
use std::process::Stdio;
use std::{io::BufReader, io::prelude::*, process::Command};

fn main() {
    let mut cmd = Command::new("target/debug/game")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("cannot spawn");

    let mut buf_reader = BufReader::new(cmd.stdout.take().expect("no stdout"));
    let mut stdin = cmd.stdin.take().expect("no stdin");
    let mut buf_string = String::new();
    let mut counter: usize = 0;

    loop {
        buf_reader.read_line(&mut buf_string).unwrap();
        print!("{}", buf_string);

        match GameState::new_from_string(&buf_string) {
            GameState::InputGuess => {
                let guess = format!("{}\n", counter).into_bytes();
                stdin.write_all(&guess).unwrap();
                counter += 1;
            }
            GameState::YouWin => break,
            _ => (),
        };

        buf_string.clear();
    }

    println!("The damn robot actually did it!");
}
