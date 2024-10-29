use crate::{
    definitions::*,
    locations::{random_location, Location},
    nations::{random_nation, Nation},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use strum::IntoEnumIterator;
use Fauna::*;
use Occupation::*;
use Resource::*;

impl GameState {
    pub fn new(name: String, nationality: Option<&'static Nation>, seed: u64) -> Self {
        let mut rng = Box::new(ChaCha8Rng::seed_from_u64(seed));
        let location = random_location(&mut rng);
        let nationality = nationality.unwrap_or_else(|| random_nation(&mut rng));

        Self {
            rng,
            player: Player::new(name, nationality),
            location,
            location_history: vec![location],
            effective_fauna: FaunaMap::default(),
            resource_change: ResourceMap::default(),
            time_spent_in_this_location: 0,
            time_passed: 0,
        }
    }

    pub fn chance(&mut self, probability: f64) -> bool {
        self.rng.gen::<f64>() <= probability
    }

    pub fn advance(&mut self, strategy: &dyn Strategy) {
        assert!(self.resource_change.values().all(|&res| res == 0.0));

        let action = strategy.take_action(self);

        let new_location = action.destination.and_then(|dest| {
            if (dest as *const Location) == (self.location as *const Location) {
                None
            } else {
                Some(dest)
            }
        });

        if let Some(dest) = new_location {
            println!(
                "{} решает сменить локацию: {} -> {}",
                self.player.name, self.location.name, dest.name
            );

            println!("{dest}");

            self.location = dest;
            self.location_history.push(dest);
            self.time_spent_in_this_location = 0;
            self.effective_fauna = dest.fauna.clone();

            if self.location_history.len() > MAX_LOCATION_HISTORY_LEN {
                let new_start = self.location_history.len() - MAX_LOCATION_HISTORY_LEN;
                self.location_history.drain(0..new_start);
            }
        } else {
            println!("{} остаётся в {}", self.player.name, self.location.name,);

            self.time_spent_in_this_location += 1;
        }

        // Пока что эффекты от локации всегда применяются до эффектов от национальности.
        // Не факт что это правильно.
        // Оставим рассуждения о том, к каким проблемам это может привести и как их избежать
        // читателю в качестве упражнения :о)
        if let Some(pre) = self.location.pre_effect {
            pre(self);
        }
        if let Some(pre) = self.player.nationality.pre_effect {
            pre(self);
        }

        if let Some(occupation) = action.occupation {
            println!("{} решает {}.", self.player.name, occupation);

            for res in self.resource_change.values_mut() {
                *res = -1.0;
            }

            let (target_resource, target_fauna) = match occupation {
                Zoombaling => (Money, Slesandra),
                Goolboning => (Satisfaction, Sisyandra),
                Shlamsing => (Health, Chuchundra),
            };

            self.resource_change[target_resource] = self.effective_fauna[target_fauna] as f64 * 2.0;
        } else {
            println!("{} не делает ничего.", self.player.name);

            for res in self.resource_change.values_mut() {
                *res = -0.5;
            }
        }

        if let Some(post) = self.location.post_effect {
            post(self, &action);
        }
        if let Some(post) = self.player.nationality.post_effect {
            post(self, &action);
        }

        println!(
            "Результаты: {Health} {:+.2}, {Money} {:+.2}, {Satisfaction} {:+.2}",
            self.resource_change[Health],
            self.resource_change[Money],
            self.resource_change[Satisfaction],
        );

        for resource in Resource::iter() {
            self.player.resources[resource] += self.resource_change[resource];
            self.resource_change[resource] = 0.0;
        }

        self.time_passed += 1;
    }
}
