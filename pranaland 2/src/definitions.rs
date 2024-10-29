use crate::{locations::Location, nations::Nation};
use enum_map::EnumMap;
use rand_chacha::ChaCha8Rng;
use std::fmt;
use strum::IntoEnumIterator;

/// Изначальное количество каждого ресурса при создании нового Игрока.
pub const STARTING_RESOURCE_AMOUNT: f64 = 10.0;

/// "Представитель разумной расы", он же Игрок.
#[derive(Clone)]
pub struct Player {
    /// Имя нашего персонажа.
    /// Может также использоваться, например, для названия файла с
    /// сохранённой игрой.
    pub name: String,

    /// Национальность персонажа.
    /// Также определяет его расу.
    pub nationality: &'static Nation,

    /// Значения всех важных ресурсов Игрока.
    pub resources: ResourceMap,
}

/// Главные жизненные ресурсы игрока - здоровье, деньги, удовлетворённость жизнью.
#[derive(Debug, Clone, Copy, PartialEq, Eq, enum_map::Enum, strum::EnumIter, strum::Display)]
pub enum Resource {
    #[strum(to_string = "Здоровье")]
    Health,

    #[strum(to_string = "Деньги")]
    Money,

    #[strum(to_string = "Удовл. жизнью")]
    Satisfaction,
}

/// Отображение из типа ресурса в его количество.
pub type ResourceMap = EnumMap<Resource, f64>;

/// Типы фауны.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, enum_map::Enum)]
pub enum Fauna {
    #[strum(to_string = "Слесандры")]
    Slesandra,

    #[strum(to_string = "Сисяндры")]
    Sisyandra,

    #[strum(to_string = "Чучундры")]
    Chuchundra,
}

/// Количество представителей для каждого типа фауны.
pub type FaunaMap = EnumMap<Fauna, usize>;

/// Эффект, применяемый на каждом временном интервале после перехода
/// в новую локацию но **ДО** того как Игрок предастся какому-либо занятию.
///
/// Параметры:
/// - текущее состояние симуляции (его можно изменять);
pub type PreEffect = dyn Fn(&mut GameState) + Sync + 'static;

/// Эффект, применяемый на каждом временном интервале после перехода
/// в новую локацию **ПОСЛЕ** того как Игрок предастся какому-либо занятию.
/// В эту функцию передаётся ссылка на уже совершённое игроком действие.
///
/// Параметры:
/// - текущее состояние симуляции (его можно изменять);
/// - действие Игрока на этом временном интервале.
pub type PostEffect = dyn Fn(&mut GameState, &Action) + Sync + 'static;

/// Максимальная длина `GameState::location_history`.
pub const MAX_LOCATION_HISTORY_LEN: usize = 10;

/// Состояние симуляции (для краткости мы назовём её "игрой").
pub struct GameState {
    /// Генератор псевдослучайных чисел.
    /// Алгоритм генерации следующего числа полностью детерминирован, 
    /// поэтому мы можем воспроизводить симуляции по зерну (seed).
    pub rng: Box<ChaCha8Rng>,

    /// Игрок - имя, нация, количество ресурсов.
    pub player: Player,

    /// Текущая локация, в которой находится Игрок.
    pub location: &'static Location,

    /// История перемещения игрока по локациям.
    /// Первые N элементов периодически удаляются для поддержания
    /// заданной максимальной длины.
    pub location_history: Vec<&'static Location>,

    /// Количество временных интервалов, которое Игрок
    /// провёл в текущей локации подряд.
    /// После перемещения Игрока в новую локацию этот счётчик
    /// обнуляется.
    pub time_spent_in_this_location: usize,

    /// Количество представителей всех типов фауны после прменения
    /// эффекта локации.
    /// В большинстве случаев сюда просто клонируется `location.fauna`
    /// после перехода в новую локацию.
    pub effective_fauna: FaunaMap,

    /// Изменение количества ресурсов Игрока в конце текущего временного интервала.
    /// В начале временного интервала сюда записываются нули (`ResourceMap::default()`).
    /// После того как Игрок выберет себе занятие, в этот асоциативный массив записываются
    /// стандартные изменение (-1 ХП, -1 ед. денег, +2 ед. удовольствия * effective_fauna[Sisyandra]).
    /// Пост-эффекты локации и национальности Игрока могут изменять это поле.
    /// После применения пост-эффектов значения из этого поля добавляются к `player.resources`,
    /// а само поле обнуляется.
    /// Это позволяет нам немного упростить написание пост-эффектов и выводить в STDOUT
    /// отчёт по изменениям ресурсов после каждого временного интервала.
    pub resource_change: ResourceMap,

    /// Количество прошедших временных интервалов с начала игры.
    /// Для первого временного интервала здесь будет записано значение `0`.
    pub time_passed: usize,
}

/// Занятие, которому Игрок может предаваться в конце каждого временного
/// интервала.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter, strum::Display)]
pub enum Occupation {
    /// Зумбалить - обменивать здоровье и радость на деньги.
    #[strum(to_string = "Зумбалить")]
    Zoombaling,

    /// Гульбонить - обменивать деньги и здоровье на удовольствия.
    #[strum(to_string = "Гульбонить")]
    Goolboning,

    /// Шлямсать - отдавать деньги и лишать себя удовольствий ради здоровья.
    #[strum(to_string = "Шлямсить")]
    Shlamsing,
}

/// Дейтствие, предпринимаемое Игроком на каждом временном интервале.
pub struct Action {
    /// Новая локация, куда перемещается Игрок в начале следующего
    /// временного интервала, либо `None`, если он остаётся в той же
    /// локации где и был.
    pub destination: Option<&'static Location>,

    /// Занятие, которому предается игрок после смены локации либо `None`,
    /// если он не делает ничего.
    pub occupation: Option<Occupation>,
}

/// Стратегия поведения игрока.
pub trait Strategy {
    /// Человекочитаемое название стратегии.
    fn name(&self) -> &'static str;

    /// Функция, выбирающаяя действие игрока на очередном шаге.
    fn take_action(&self, state: &mut GameState) -> Action;
}

impl Player {
    pub fn new(name: String, nationality: &'static Nation) -> Self {
        Self {
            name,
            nationality,
            resources: Resource::iter()
                .map(|res| (res, STARTING_RESOURCE_AMOUNT))
                .collect(),
        }
    }

    /// Игрок погибает, если количество любого его ресурса
    /// становится равным или меньшим нуля.
    pub fn is_dead(&self) -> bool {
        self.resources.values().any(|&x| x <= 0.0)
    }

    pub fn is_alive(&self) -> bool {
        !self.is_dead()
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Resource::*;
        write!(
            f,
            "{} [{}] | {Health}: {:.02}, {Money}: {:.02}, {Satisfaction}: {:.02}",
            self.name,
            self.nationality.name_singular,
            self.resources[Health],
            self.resources[Money],
            self.resources[Satisfaction],
        )
    }
}
