use crate::{definitions::{Action, GameState, Occupation, Strategy}, locations::random_location};
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

pub struct ApatheticStrategy;

impl Strategy for ApatheticStrategy {
    fn name(&self) -> &'static str {
        "Апатичная стратегия"
    }

    fn take_action(&self, _state: &mut GameState) -> Action {
        Action {
            destination: None,
            occupation: None,
        }
    }
}

pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn name(&self) -> &'static str {
        "Случайная стратегия"
    }

    fn take_action(&self, state: &mut GameState) -> Action {
        let destination = Some(random_location(&mut state.rng));
        let occupation = Occupation::iter().choose(&mut state.rng);
        Action {
            destination,
            occupation,
        }
    }
}
