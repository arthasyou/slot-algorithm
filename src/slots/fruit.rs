use rand::Rng;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum GeneralLevel {
    High,    // 最高赔率等级
    Medium,  // 中等赔率等级
    Low,     // 较低赔率等级
    Minimal, // 最小赔率等级
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FruitReward {
    pub symbol: FruitSymbol,
    pub bet: u64,
    pub reward: u64,
    pub flag: bool,
}

impl FruitReward {
    pub fn new(symbol: FruitSymbol, bet: u64, reward: u64, flag: bool) -> Self {
        Self {
            symbol,
            bet,
            reward,
            flag,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FruitBet {
    pub symbol: FruitSymbol,
    pub value: u64,
}

impl FruitBet {
    pub fn new(symbol: FruitSymbol, value: u64) -> Self {
        Self { symbol, value }
    }

    fn draw(&self, level: &GeneralLevel, pool: &mut Pool) -> (bool, u64) {
        let odds = self.symbol.get_odds(level) as u64;
        pool.draw(self.value, odds)
    }
}

pub fn draw(fruits: Vec<FruitBet>, pool: &mut Pool) -> Vec<FruitReward> {
    let level = random_level(); // 获取一次 level
    fruits
        .into_iter()
        .map(|bet| {
            let (flag, reward) = bet.draw(&level, pool);
            FruitReward::new(bet.symbol, bet.value, reward, flag) // 创建 FruitReward
        })
        .collect()
}

pub fn random_level() -> GeneralLevel {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => GeneralLevel::High,
        1 => GeneralLevel::Medium,
        2 => GeneralLevel::Low,
        _ => GeneralLevel::Minimal,
    }
}
