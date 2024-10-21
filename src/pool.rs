use crate::wave;
use rand::{rngs::StdRng, Rng, SeedableRng};

const ASCENT_SPEED_RATE: u64 = 2000;
const SPEED_RATE: u64 = 1000;
const SPEED_BIG: u64 = 8000;
const BIG_ODDS: u64 = 50;
pub const RATIO: u64 = 10000; //比率 万分比

#[derive(Debug, Clone)]
pub struct Pool {
    pub id: u32,              // ID
    pub owner_id: u32,        // 所有者 ID
    pub bet_unit: u64,        // 每分价值
    pub base_line: u64,       // 底线
    pub boundary: u64,        // 边界线
    pub brokerage_ratio: u64, // 佣金比率
    pub jackpot_ratio: u64,   // 彩金比率
    pub pot_ratio: u64,       // 池底比率
    pub pot: u64,             // 当前池底
    pub jackpot: u64,         // 彩金
    pub suction: u64,         // 吸码量
    pub brokerage: u64,       // 佣金
    pub bonus: u64,           // 总赢分
    pub advance: u64,         // 垫分
    waves: Vec<u64>,          // 波浪
    segment: (u64, u64),      // 分段
    rng: StdRng,
}

impl Pool {
    /// 初始化一个新的 Pool 实例
    pub fn new(
        id: u32,
        owner_id: u32,
        bet_unit: u64,
        brokerage_ratio: u64,
        jackpot_ratio: u64,
        boundary: u64,
        advance: u64,
    ) -> Self {
        let pot = advance;
        let boundary = boundary;
        create_pool(
            id,
            owner_id,
            bet_unit,
            0,
            boundary,
            brokerage_ratio,
            jackpot_ratio,
            pot,
            0,
            0,
            0,
            0,
            pot,
        )
    }

    pub fn load_pool(
        id: u32,
        owner_id: u32,
        bet_unit: u64,
        base_line: u64,
        boundary: u64,
        brokerage_ratio: u64,
        jackpot_ratio: u64,
        pot: u64,
        jackpot: u64,
        suction: u64,
        brokerage: u64,
        bonus: u64,
        advance: u64,
    ) -> Self {
        create_pool(
            id,
            owner_id,
            bet_unit,
            base_line,
            boundary,
            brokerage_ratio,
            jackpot_ratio,
            pot,
            jackpot,
            suction,
            brokerage,
            bonus,
            advance,
        )
    }

    /// 根据传入的 WaveState 执行 draw 方法，并返回命中结果和 reward 值
    pub fn draw(&mut self, bets: u64, odds: u64) -> (bool, u64) {
        let state = self.get_state();
        println!("{:?}", state);
        self.update_pool_with_bets(bets);
        let reward = self.calculate_reward(bets, odds);

        let hit = match state {
            WaveState::Ascent => self.ascent(odds, reward),
            WaveState::Fall => self.fall(odds, reward),
        };

        if hit {
            (true, reward)
        } else {
            (false, 0) // 未命中时返回 0
        }
    }

    pub fn get_mut_rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    /// 更新 brokerage_ratio 和 pot_ratio，确保它们之和等于 RATIO
    pub fn update_ratios(&mut self, new_brokerage_ratio: u64) {
        self.brokerage_ratio = new_brokerage_ratio;
        self.pot_ratio = RATIO - new_brokerage_ratio;
    }

    pub fn get_segment(&self) -> (u64, u64) {
        self.segment
    }

    pub fn get_waves_len(&self) -> usize {
        self.waves.len()
    }
}

fn create_pool(
    id: u32,
    owner_id: u32,
    bet_unit: u64,
    base_line: u64,
    boundary: u64,
    brokerage_ratio: u64,
    jackpot_ratio: u64,
    pot: u64,
    jackpot: u64,
    suction: u64,
    brokerage: u64,
    bonus: u64,
    advance: u64,
) -> Pool {
    let mut waves = wave::create_wave(pot, 0, boundary);
    let segment = wave::create_segment(&mut waves, pot);
    let rng = StdRng::from_entropy();
    Pool {
        id,
        owner_id,
        bet_unit,
        brokerage_ratio,
        jackpot_ratio,
        base_line,
        boundary,
        pot_ratio: RATIO - brokerage_ratio - jackpot_ratio,
        pot,
        jackpot,
        suction,
        brokerage,
        bonus,
        advance,
        waves,
        segment,
        rng,
    }
}

impl Pool {
    /// 更新池底金额及相关属性
    fn update_pool_with_bets(&mut self, bet: u64) {
        self.suction += bet;
        self.pot += self.pot_ratio * bet;
        self.brokerage += self.brokerage_ratio * bet;
        self.jackpot += self.jackpot_ratio * bet;
    }

    /// 计算当前下注及赔率的奖励
    fn calculate_reward(&self, bets: u64, odds: u64) -> u64 {
        bets * odds * RATIO
    }

    /// 上升逻辑处理，根据状态决定是否减少池底或调整波浪，返回是否命中
    fn ascent(&mut self, odds: u64, reward: u64) -> bool {
        if self.analyzing_ascent(reward) && self.ascent_run(odds) {
            self.decrease_pot(reward);
            true
        } else {
            self.ascent_action();
            false
        }
    }

    /// 检查是否符合上升条件
    fn analyzing_ascent(&self, reward: u64) -> bool {
        let (bottom, _) = self.segment;

        // 检查是否会发生溢出
        if self.pot < reward {
            return false; // 如果 reward 大于 pot，则不符合条件
        }

        bottom < self.pot - reward
    }

    /// 上升时执行的奖励计算及判定
    fn ascent_run(&mut self, odds: u64) -> bool {
        let new_odds = odds * (ASCENT_SPEED_RATE + RATIO); // 计算并放大到万分比表示
        self.run(new_odds)
    }

    /// 当上升条件未达到时，执行波浪调整
    fn ascent_action(&mut self) {
        let (_, destination) = self.segment;
        let pot = self.pot;
        if pot > destination {
            self.consume_and_segment();
        }
    }

    /// 下降逻辑处理，根据状态决定是否减少池底或调整波浪，返回是否命中
    fn fall(&mut self, odds: u64, reward: u64) -> bool {
        match self.analyzing_fall(reward) {
            FallState::Normal => {
                if self.fall_run(odds) {
                    self.fall_action(reward);
                    true
                } else {
                    false
                }
            }
            FallState::Win => {
                self.fall_action(reward);
                true
            }
            FallState::Reflesh => {
                self.create_new_wave_and_segment();
                false
            }
        }
    }

    fn analyzing_fall(&self, reward: u64) -> FallState {
        match reward > self.pot {
            true => {
                let (top, _) = self.segment;
                match self.pot > top {
                    true => FallState::Win,
                    false => FallState::Normal,
                }
            }
            false => FallState::Reflesh,
        }
    }

    /// 下降时执行的奖励计算及判定
    fn fall_run(&mut self, odds: u64) -> bool {
        let new_odds = if odds >= BIG_ODDS {
            odds * (RATIO - SPEED_BIG)
        } else {
            odds * (RATIO - SPEED_RATE)
        };
        self.run(new_odds)
    }

    /// 执行下降操作，更新池底及波浪
    fn fall_action(&mut self, reward: u64) {
        self.decrease_pot(reward);
        let (_, destination) = self.segment;
        if self.pot <= destination {
            self.consume_and_segment();
        }
    }

    /// 生成随机数判断胜负
    fn run(&mut self, odds: u64) -> bool {
        // let mut rng = self.rng.lock().unwrap();
        let rand = self.rng.gen_range(1..=odds);
        rand <= RATIO
    }

    /// 减少池底并将相应金额加入奖金，确保池底不会低于零
    fn decrease_pot(&mut self, reward: u64) {
        if self.pot >= reward {
            self.pot -= reward;
            self.bonus += reward;
        } else {
            // 如果 reward 超过了 pot，仅能扣除 pot 的全部值，并增加相应的 bonus
            self.bonus += self.pot;
            self.pot = 0;
        }
    }

    /// 从波浪中获取第一个元素并创建分段，如果波浪为空则创建新波浪
    fn consume_and_segment(&mut self) {
        if let Some(wave) = self.waves.first().cloned() {
            self.waves.remove(0); // 删除第一个元素
            self.create_segment(wave);

            // 如果 waves 已空，则创建新的波浪
            if self.waves.is_empty() {
                println!("waves: {:?}", &self.waves);
                self.create_wave();
            }
        }
    }

    /// 生成新的波浪及分段
    fn create_new_wave_and_segment(&mut self) {
        self.create_wave(); // 先创建新的波浪
        self.consume_and_segment(); // 调用通用函数处理
    }

    /// 创建新的分段
    fn create_segment(&mut self, wave: u64) {
        let segment = (self.pot, wave);
        self.segment = segment;
    }

    /// 创建新的波浪
    fn create_wave(&mut self) {
        let waves = wave::create_wave(self.pot, self.base_line, self.boundary);
        self.waves = waves;
    }

    fn get_state(&self) -> WaveState {
        let (_, destination) = self.segment;
        if self.pot > destination {
            WaveState::Fall
        } else {
            WaveState::Ascent
        }
    }
}

#[derive(Debug)]
pub enum WaveState {
    Ascent,
    Fall,
}

#[derive(Debug)]
pub enum FallState {
    Normal,
    Win,
    Reflesh,
}
