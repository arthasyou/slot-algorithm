use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::pool::Pool;

const BAR_HIGH_ODDS: u8 = 100;
const BAR_MEDIUM_ODDS: u8 = 50;
const BAR_LOW_ODDS: u8 = 25;

const GENERAL_HIGH_ODDS: u8 = 40;
const GENERAL_MEDIUM_ODDS: u8 = 30;
const GENERAL_LOW_ODDS: u8 = 20;
const GENERAL_MINIMAL_ODDS: u8 = 2;

const SECONDARY_HIGH_ODDS: u8 = 20;
const SECONDARY_MEDIUM_ODDS: u8 = 15;
const SECONDARY_LOW_ODDS: u8 = 10;
const SECONDARY_MINIMAL_ODDS: u8 = 2;

const APPLE_ODDS: u8 = 5;

const BAR_POSITION_HIGH: u8 = 3;
const BAR_POSITION_MID: u8 = 2;
const BAR_POSITION_LOW: u8 = 4;

const LUCKY_SEVEN_POSITION_BIG: u8 = 15;
const LUCKY_SEVEN_POSITION_MINIMA: u8 = 14;

const STAR_POSITION_BIG: u8 = 19;
const STAR_POSITION_MINIMA: u8 = 20;

const WATERMELON_POSITION_BIG: u8 = 7;
const WATERMELON_POSITION_MINIMA: u8 = 8;

const BELL_POSITION_BIG: [u8; 2] = [1, 13];
const BELL_POSITION_MINIMA: u8 = 23;

const LEMON_POSITION_BIG: [u8; 2] = [6, 18];
const LEMON_POSITION_MINIMA: u8 = 17;

const ORANGE_POSITION_BIG: [u8; 2] = [0, 12];
const ORANGE_POSITION_MINIMA: u8 = 11;

const APPLE_POSITION: [u8; 4] = [5, 10, 16, 22];

const NONE: u8 = 21;
const MULTIMPLE: u8 = 9;

#[derive(Debug)]
pub enum GeneralLevel {
    High,    // 最高赔率等级
    Medium,  // 中等赔率等级
    Low,     // 较低赔率等级
    Minimal, // 最小赔率等级
}

impl GeneralLevel {
    fn get_position(&self) -> u8 {
        match self {
            GeneralLevel::High => 0,
            GeneralLevel::Medium => 1,
            _ => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum FruitSymbol {
    Bar,
    LuckySeven,
    Star,
    Watermelon,
    Bell,
    Lemon,
    Orange,
    Apple,
}

impl FruitSymbol {
    fn get_odds(&self, level: &GeneralLevel) -> u8 {
        match self {
            FruitSymbol::Bar => match level {
                GeneralLevel::High => BAR_HIGH_ODDS,
                GeneralLevel::Medium => BAR_MEDIUM_ODDS,
                _ => BAR_LOW_ODDS,
            },
            FruitSymbol::LuckySeven | FruitSymbol::Star | FruitSymbol::Watermelon => match level {
                GeneralLevel::High => GENERAL_HIGH_ODDS,
                GeneralLevel::Medium => GENERAL_MEDIUM_ODDS,
                GeneralLevel::Low => GENERAL_LOW_ODDS,
                GeneralLevel::Minimal => GENERAL_MINIMAL_ODDS,
            },
            FruitSymbol::Bell | FruitSymbol::Lemon | FruitSymbol::Orange => match level {
                GeneralLevel::High => SECONDARY_HIGH_ODDS,
                GeneralLevel::Medium => SECONDARY_MEDIUM_ODDS,
                GeneralLevel::Low => SECONDARY_LOW_ODDS,
                GeneralLevel::Minimal => SECONDARY_MINIMAL_ODDS,
            },
            FruitSymbol::Apple => APPLE_ODDS,
        }
    }

    fn get_position(&self, level: &GeneralLevel, rng: &mut StdRng) -> u8 {
        match self {
            FruitSymbol::Bar => match level {
                GeneralLevel::High => BAR_POSITION_HIGH,
                GeneralLevel::Medium => BAR_POSITION_MID,
                _ => BAR_POSITION_LOW,
            },
            FruitSymbol::LuckySeven => match level {
                GeneralLevel::Minimal => LUCKY_SEVEN_POSITION_MINIMA,
                _ => LUCKY_SEVEN_POSITION_BIG,
            },
            FruitSymbol::Star => match level {
                GeneralLevel::Minimal => STAR_POSITION_MINIMA,
                _ => STAR_POSITION_BIG,
            },
            FruitSymbol::Watermelon => match level {
                GeneralLevel::Minimal => WATERMELON_POSITION_MINIMA,
                _ => WATERMELON_POSITION_BIG,
            },
            FruitSymbol::Bell => match level {
                GeneralLevel::Minimal => BELL_POSITION_MINIMA,
                _ => match rng.gen_range(0..2) {
                    0 => BELL_POSITION_BIG[0],
                    _ => BELL_POSITION_BIG[1],
                },
            },
            FruitSymbol::Lemon => match level {
                GeneralLevel::Minimal => LEMON_POSITION_MINIMA,
                _ => match rng.gen_range(0..2) {
                    0 => LEMON_POSITION_BIG[0],
                    _ => LEMON_POSITION_BIG[1],
                },
            },
            FruitSymbol::Orange => match level {
                GeneralLevel::Minimal => ORANGE_POSITION_MINIMA,
                _ => match rng.gen_range(0..2) {
                    0 => ORANGE_POSITION_BIG[0],
                    _ => ORANGE_POSITION_BIG[1],
                },
            },
            FruitSymbol::Apple => match rng.gen_range(0..4) {
                0 => APPLE_POSITION[0],
                1 => APPLE_POSITION[1],
                2 => APPLE_POSITION[2],
                _ => APPLE_POSITION[3],
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FruitBet {
    pub symbol: FruitSymbol,
    #[validate(range(min = 1, max = 100, message = "Amount must be between 1 and 100"))]
    pub value: u32,
}

impl FruitBet {
    fn draw(&self, level: &GeneralLevel, pool: &mut Pool) -> (bool, u64, Option<u8>) {
        let odds = self.symbol.get_odds(level) as u64;
        let (flag, reward) = pool.draw(self.value as u64, odds);
        let position = match flag {
            true => Some(get_furit_postition(&self.symbol, level, pool.get_mut_rng())),
            false => None,
        };
        (flag, reward, position)
    }
}

fn get_furit_postition(symbol: &FruitSymbol, level: &GeneralLevel, rng: &mut StdRng) -> u8 {
    symbol.get_position(level, rng)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FruitReward {
    pub symbol: FruitSymbol,
    pub bet: u64,
    pub reward: u64,
    pub flag: bool,
}

impl FruitReward {
    fn new(symbol: FruitSymbol, bet: u64, reward: u64, flag: bool) -> Self {
        Self {
            symbol,
            bet,
            reward,
            flag,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FruitDraw {
    pub rewards: Vec<FruitReward>,
    pub positions: Vec<u8>,
    pub odds: u8,
}

pub fn draw(fruits: Vec<FruitBet>, pool: &mut Pool) -> FruitDraw {
    let level = random_level(pool.get_mut_rng()); // 获取一次 level
    let mut positions = Vec::new();
    let mut missed = full_symbol();
    let rewards = calculate_rewards(fruits, pool, &level, &mut positions, &mut missed);
    let new_positions = match positions.len() {
        0 => match missed.len() {
            0 => vec![NONE],
            _ => get_missed_position(&missed, pool.get_mut_rng()),
        },
        1 => positions,
        _ => {
            positions.insert(0, MULTIMPLE);
            positions
        }
    };
    FruitDraw {
        rewards,
        positions: new_positions,
        odds: level.get_position(),
    }
}

pub fn random_level(rng: &mut StdRng) -> GeneralLevel {
    // let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => GeneralLevel::High,
        1 => GeneralLevel::Medium,
        2 => GeneralLevel::Low,
        _ => GeneralLevel::Minimal,
    }
}

fn full_symbol() -> Vec<FruitSymbol> {
    vec![
        FruitSymbol::Bar,
        FruitSymbol::LuckySeven,
        FruitSymbol::Star,
        FruitSymbol::Watermelon,
        FruitSymbol::Bell,
        FruitSymbol::Lemon,
        FruitSymbol::Orange,
        FruitSymbol::Apple,
    ]
}

fn remove_symbol(symbols: &mut Vec<FruitSymbol>, target: &FruitSymbol) {
    symbols.retain(|s| s != target);
}

fn calculate_rewards(
    fruits: Vec<FruitBet>,
    pool: &mut Pool,
    level: &GeneralLevel,    // 假设 random_level 返回的类型为 LevelType
    positions: &mut Vec<u8>, // 假设 positions 是 u8 类型
    missed: &mut Vec<FruitSymbol>,
) -> Vec<FruitReward> {
    fruits
        .into_iter()
        .map(|bet| {
            let (flag, reward, position) = bet.draw(level, pool);
            if let Some(p) = position {
                positions.push(p);
            }
            remove_symbol(missed, &bet.symbol); // 从 missed 中删除符号
            FruitReward::new(bet.symbol, bet.value as u64, reward, flag)
        })
        .collect()
}

fn get_missed_position(missed: &Vec<FruitSymbol>, rng: &mut StdRng) -> Vec<u8> {
    let symbol = missed.choose(rng).unwrap();
    let level = match rng.gen_range(0..2) {
        0 => GeneralLevel::Minimal,
        _ => GeneralLevel::Low,
    };
    let pos = symbol.get_position(&level, rng);
    vec![pos]
}
