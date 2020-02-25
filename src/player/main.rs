mod game_state;

use game_state::GameState;
use std::process::Stdio;
use tokio::{io::BufReader, prelude::*, process::Command};

#[tokio::main]
async fn main() {
    let mut cmd = Command::new("target/debug/game")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("cannot spawn");

    let stdout = cmd.stdout.take().expect("no stdout");
    let mut stdin = cmd.stdin.take().expect("no stdin");
    let mut buf_reader = BufReader::new(stdout);
    let mut buf_string = String::new();

    let mut counter: usize = 0;

    loop {
        buf_reader.read_line(&mut buf_string).await.unwrap();
        let game_state = GameState::new_from_string(&buf_string);

        match game_state {
            GameState::InputGuess => {
                print!("{}", buf_string);

                let guess = format!("{}\n", counter).into_bytes();
                stdin.write(&guess).await.unwrap();
                counter += 1;
            }
            GameState::YouWin => break,
            _ => print!("{}", &buf_string),
        };

        buf_string.clear();
    }

    println!("The damn robot actually did it!");
}
