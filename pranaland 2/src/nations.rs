use crate::definitions::*;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

use Fauna::*;
use Occupation::*;
use Race::*;
use Resource::*;

/// Расы разумных существ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Race {
    Shlendrick,
    Hipstick,
    Scoofick,
}

/// Нации разумных существ.
pub struct Nation {
    /// Раса.
    pub race: Race,

    /// Название нации во множественном числе ("Шведы").
    pub name_plural: &'static str,

    /// Название нации в единственном числе ("Швед").
    pub name_singular: &'static str,

    /// Эффект, применяющийся на каждом временном интервале
    /// ДО действия Игрока, если он относится к этой нации.
    pub pre_effect: Option<&'static PreEffect>,

    /// Эффект, применяющийся на каждом временном интервале
    /// ПОСЛЕ действия Игрока, если он относится к этой нации.
    pub post_effect: Option<&'static PostEffect>,
}

lazy_static! {
    pub static ref NATIONS: Vec<Nation> = vec![
        Nation {
            name_plural: "Можоры",
            name_singular: "Можор",
            race: Shlendrick,
            pre_effect: None,
            post_effect: Some(&|state, action| {
                // При гульбонстве тратят на 23%  больше денег по сравнению с остальными,
                // зато при зумбальстве в одном случае из 3 вообще не расходуют здоровье.
                match action.occupation {
                    Some(Goolboning) => {
                        println!(
                            "Как {}, {} расходует больше денег на гульбонство.",
                            state.player.name,
                            state.player.nationality.name_singular
                        );
                        state.resource_change[Money] *= 1.23;
                    }
                    Some(Zoombaling) => {
                        if state.chance(1.0 / 3.0) {
                            println!(
                                "{} умудряется уклониться от работы и не потратить своё здоровье! ({})",
                                state.player.name,
                                state.player.nationality.name_singular
                            );
                            state.resource_change[Health] = 0.0;
                        }
                    }
                    _ => {},
                }
            }),
        },
        Nation {
            name_plural: "Нищебороды",
            name_singular: "Нищебород",
            race: Shlendrick,
            pre_effect: None,
            post_effect: Some(&|state, action| {
                // При гульбонстве тратят на 87% меньше денег,
                // но на 76% больше здоровья.
                if action.occupation == Some(Goolboning) {
                    println!(
                        "Так как {} - {}, он тратит гораздо меньше денег и гораздо больше здоровья на гульбонство.",
                        state.player.name,
                        state.player.nationality.name_singular
                    );
                    state.resource_change[Money] *= 1.0 - 0.87;
                    state.resource_change[Health] *= 1.76;
                }
            }),
        },
        Nation {
            name_plural: "Соевые",
            name_singular: "Соевый",
            race: Hipstick,
            pre_effect: None,
            post_effect: Some(&|state, action| {
                // Крайне тяжело переносят зумбальство, затрачивая дополнительно
                // 0.12 единиц здоровья на каждую чучундру (???) в локации.
                if action.occupation == Some(Zoombaling) {
                    const PENALTY: f64 = 0.12;
                    let chuch_count = state.effective_fauna[Chuchundra];
                    let change = chuch_count as f64 * PENALTY;
                    println!(
                        "Как {}, {} плохо переносит зумбальство и тратит дополнительно по {} ед. здоровья \
                        на каждую чучундру ({}) в локации ({}).",
                        state.player.nationality.name_singular,
                        state.player.name,
                        PENALTY,
                        chuch_count,
                        -change
                    );
                    state.resource_change[Health] -= change;
                }
            }),
        },
        Nation {
            name_plural: "Просветлённые",
            name_singular: "Просветлённый",
            race: Hipstick,
            pre_effect: None,
            post_effect: Some(&|state, action| {
                // Во время шлямсания могут получить дополнтельную удовлетворенность жизнью в количестве,
                // равном количеству сисяндр в последних 3 локациях, умноженному на 0.31.
                const N: usize = 3;
                const MULTIPLIER: f64 = 0.31;

                if action.occupation == Some(Shlamsing) {
                    let loc_iter = state.location_history.iter()
                        .rev()
                        .take(N);

                    let sis_count: usize = loc_iter
                        .clone()
                        .map(|loc| loc.count(Sisyandra))
                        .sum();

                    let satisfaction_bonus = sis_count as f64 * 0.31;

                    println!(
                        "{} - {}, и поэтому во время шлямсания он получает дополнительную удовлетворённость \
                         жизнью от количества сисяндр в последних {N} локациях: {} (итого {}), умноженного \
                         на {MULTIPLIER:.2}: {:.2}.",
                        state.player.name,
                        state.player.nationality.name_singular,
                        loc_iter
                            .map(|loc| format!("{} - {}", loc.name, loc.fauna[Sisyandra]))
                            .collect::<Vec<_>>()
                            .join(", "),
                        sis_count,
                        satisfaction_bonus
                    );

                    state.resource_change[Satisfaction] += satisfaction_bonus;
                }
            }),
        },
        Nation {
            name_plural: "Дроценты",
            name_singular: "Дроцент",
            race: Scoofick,
            pre_effect: None,
            post_effect: Some(&|state, action| {
                // Практически не умеют гульбонить, затрачивая вполовину меньше здоровья и денег,
                // и получая вполовину меньше удовлетворенности.
                if action.occupation == Some(Goolboning) {
                    println!(
                        "{} - {}, и потому гульбонит вполсилы.",
                        state.player.name,
                        state.player.nationality.name_singular,
                    );

                    state.resource_change[Health] *= 0.5;
                    state.resource_change[Money] *= 0.5;
                    state.resource_change[Satisfaction] *= 0.5;
                }
            }),
        },
        Nation {
            name_plural: "Железноухие",
            name_singular: "Железноухий",
            race: Scoofick,
            pre_effect: None,
            post_effect: Some(&|state, action| {
                // Не расходуют удовлетворенность жизнью при зумбальстве, зато с вероятностью 0.33
                // не получают денег от каждой слесандры в локации.
                if action.occupation == Some(Zoombaling) {
                    println!(
                        "{} - {}, и потому не расходует здоровье при зумбальстве.",
                        state.player.name,
                        state.player.nationality.name_singular,
                    );

                    state.resource_change[Health] = 0.0;

                    let sles_count = state.effective_fauna[Slesandra];
                    for i in 0..sles_count {
                        if state.chance(0.33) {
                            println!("Слесандра №{} оставила нашего железноухого без денег!", i + 1);
                            state.resource_change[Money] -= 2.0;
                        }
                    }
                }
            }),
        },
    ];
}

pub fn random_nation(rng: &mut impl rand::Rng) -> &'static Nation {
    NATIONS.choose(rng).expect("NATIONS Vec can't be empty!")
}

pub fn find_nation(name: &str) -> Option<&'static Nation> {
    NATIONS
        .iter()
        .find(|nation| nation.name_singular == name || nation.name_plural == name)
}
