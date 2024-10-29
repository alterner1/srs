use crate::definitions::{
    Fauna::{self, *},
    FaunaMap,
    Occupation::*,
    PostEffect, PreEffect,
    Resource::*,
};
use enum_map::enum_map;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::fmt;

use Biome::*;

/// То, что в условии называется "Локацией".
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum Biome {
    #[strum(to_string = "Воркленд")]
    Workland,

    #[strum(to_string = "Бичленд")]
    Beachland,

    #[strum(to_string = "Праналенд")]
    Pranaland,
}

/// Локация - место, где Игрок может находиться.
/// То, что в условии называется "Местностью".
pub struct Location {
    /// Человекочитаемое название локации, например "Балбесбург".
    pub name: &'static str,

    /// Биом ("Локация" в условии), где находится эта локация.
    pub biome: Biome,

    /// Количество представителей для каждого типа фауны.
    pub fauna: FaunaMap,

    pub effect_description: &'static str,
    pub pre_effect: Option<&'static PreEffect>,
    pub post_effect: Option<&'static PostEffect>,
}

lazy_static! {
    static ref WORKLAND_FAUNA: FaunaMap = enum_map! {
        Slesandra => 3,
        Sisyandra => 1,
        Chuchundra => 1,
    };
    static ref BEACHLAND_FAUNA: FaunaMap = enum_map! {
        Slesandra => 1,
        Sisyandra => 3,
        Chuchundra => 1,
    };
    static ref PRANALAND_FAUNA: FaunaMap = enum_map! {
        Slesandra => 1,
        Sisyandra => 1,
        Chuchundra => 3,
    };
    pub static ref LOCATIONS: Vec<Location> = vec![
        Location {
            name: "Балбесбург",
            biome: Workland,
            fauna: *WORKLAND_FAUNA,
            effect_description: "С вероятноятью 0.15 каждая слесандра может \
                                 нанести ущерб здоровью в размере 0.1 единицы.",
            pre_effect: None,
            post_effect: Some(&|state, _action| {
                const DAMAGE: f64 = 0.1;
                for i in 0..state.effective_fauna[Slesandra] {
                    if state.chance(0.15) {
                        println!("Слесандра №{} причиняет вред здоровью нашего героя в размере {DAMAGE:.1} ед.", i + 1);
                        state.resource_change[Health] -= 0.1;
                    }
                }
            })
        },
        Location {
            name: "Долбесбург",
            biome: Workland,
            fauna: *WORKLAND_FAUNA,
            effect_description: "Добавляет 20% к производительности слесандр, \
                                 но забирает на 30% больше удовлетворенности.",
            pre_effect: None,
            post_effect: Some(&|state, action| {
                if action.occupation == Some(Zoombaling) {
                    state.resource_change[Money] *= 1.2;
                    state.resource_change[Satisfaction] *= 1.3;
                    assert!(state.resource_change[Satisfaction] <= 0.0);
                }
            })
        },
        Location {
            name: "Курамарибы",
            biome: Beachland,
            fauna: *BEACHLAND_FAUNA,
            effect_description: "Каждая сисяндра перестает работать с вероятностью 0.7 \
                                 во втором и последующих интервалах нахождения в локации.",
            pre_effect: Some(&|state| {
                if state.time_spent_in_this_location >= 1 {
                    let sis_count = state.effective_fauna[Sisyandra];
                    for _ in 0..sis_count {
                        if state.chance(0.7) {
                            state.effective_fauna[Sisyandra] -= 1;
                            println!(
                                "Одна из сисяндр перестаёт работать (осталось {}).",
                                state.effective_fauna[Sisyandra]
                            );
                        }
                    }
                }
            }),
            post_effect: None,
        },
        Location {
            name: "Пунта-пеликана",
            biome: Beachland,
            fauna: *BEACHLAND_FAUNA,
            effect_description: "Начиная со 2 интервала нахождения в локации, сисяндры \
                                 генерируют на 23% больше удовлетворенности, но с \
                                 вероятностью 0.2 списывается 50% ВСЕХ денег.",
            pre_effect: None,
            post_effect: Some(&|state, action| {
                if state.time_spent_in_this_location >= 1 {
                    if action.occupation == Some(Goolboning) {
                        state.resource_change[Satisfaction] *= 1.23;
                    }

                    if state.chance(0.2) {
                        let money_lost = state.player.resources[Money] * 0.5; 
                        state.resource_change[Money] -= money_lost;

                        println!(
                            "{} проигрывает в казино и теряет половину денег! ({:.2})",
                            state.player.name,
                            -money_lost
                        );
                    }
                }
            })
        },
        Location {
            name: "Шринавас",
            biome: Pranaland,
            fauna: *PRANALAND_FAUNA,
            effect_description: "Добавляет 13 процентов к производительности чучундр.",
            pre_effect: None,
            post_effect: Some(&|state, action| {
                if action.occupation == Some(Shlamsing) {
                    state.resource_change[Health] *= 1.13;
                }
            })
        },
        Location {
            name: "Харе-Кириши",
            biome: Pranaland,
            fauna: *PRANALAND_FAUNA,
            effect_description: "При попадании Дроцентов они расходуют дополнительно \
                                 по 10% здоровья за каждый интервал.",
            pre_effect: None,
            post_effect: Some(&|state, _action| {
                if state.player.nationality.name_singular == "Дроцент" {
                    const PENALTY: f64 = 0.1;
                    let damage = state.player.resources[Health] * 0.1;
                    state.resource_change[Health] -= damage;

                    println!(
                        "{} - {}, и, пока находится в {}, получает урон в размере {PENALTY:.2} от своего здоровья: {:.2}.",
                        state.player.name,
                        state.player.nationality.name_singular,
                        state.location.name,
                        damage
                    );
                }
            })
        },
    ];
}

pub fn random_location(rng: &mut impl rand::Rng) -> &'static Location {
    LOCATIONS
        .choose(rng)
        .expect("LOCATIONS Vec can't be empty!")
}

impl Location {
    pub fn count(&self, fauna: Fauna) -> usize {
        self.fauna[fauna]
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): {Slesandra}: {}, {Sisyandra}: {}, {Chuchundra}: {}\n{}",
            self.name,
            self.biome,
            self.count(Slesandra),
            self.count(Sisyandra),
            self.count(Chuchundra),
            self.effect_description
        )
    }
}
