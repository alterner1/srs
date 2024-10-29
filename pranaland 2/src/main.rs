#![allow(dead_code)]

use crate::{definitions::Strategy, nations::find_nation, strategies::*};
use definitions::GameState;

mod definitions;
mod locations;
mod nations;
mod simulation;
mod strategies;

fn main() {
    // let nation = None;
    let nation = Some(find_nation("Просветлённый").unwrap());

    let mut state = GameState::new("Жора".to_string(), nation, 1337);

    // let strategy = ApatheticStrategy;
    let strategy = RandomStrategy;
    println!("Используется {}.", strategy.name());

    while state.player.is_alive() {
        println!(
            "{:-^80}",
            format!(" Интервал времени №{} ", state.time_passed + 1)
        );
        println!("{}", state.player);
        println!("Текущая локация: {}", state.location.name);

        state.advance(&strategy);

        pause();
    }

    println!(
        "{} мёртв :о(\nОн прожил {} временных интервалов.",
        state.player.name, state.time_passed
    );
}

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
fn pause() {
    use std::io::{self, Read, Write};

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Нажмите любую клавишу...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
