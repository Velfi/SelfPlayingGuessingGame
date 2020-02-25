pub enum GameState {
    Start,
    InputGuess,
    YouGuessed,
    TooBig,
    TooSmall,
    YouWin,
}

impl GameState {
    pub fn new_from_string(string: &str) -> Self {
        match string {
            "Guess the number!\n" => Self::Start,
            "Please input your guess.\n" => Self::InputGuess,
            "Too big!\n" => Self::TooBig,
            "Too small!\n" => Self::TooSmall,
            "You win!\n" => Self::YouWin,
            text if text.starts_with("You guessed:") => Self::YouGuessed,
            _ => unreachable!(format!("Couldn't create GameState from \"{:?}\"", string)),
        }
    }
}
