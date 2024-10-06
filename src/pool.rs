#[derive(Debug, Clone)]
pub struct Pool {
    id: u32,              // ID
    bet_unit: u64,        // 底分
    ratio: u64,           // 比率 (100=百分比，10000=万分比)
    brokerage_ratio: u64, // 佣金比率
    pot_ratio: u64,       // 池底比率, 一分等于多少钱
    pot: u64,             // 当前池底
    base_line: u64,       // 底线
    boundary: u64,        // 边界线
    suction: u64,         // 吸码量
    bonus: u64,           // 总赢分
    jackpot: u64,         // 彩金
    advance: u64,         // 垫分
    wave: u64,            // 波浪
    segment: u64,         // 分段
}

impl Pool {
    pub fn new(
        id: u32,
        bet_unit: u64,
        ratio: u64,

        brokerage_ratio: u64,
        pot_ratio: u64,
        pot: u64,
        base_line: u64,
        boundary: u64,
        suction: u64,
        bonus: u64,
        jackpot: u64,
        advance: u64,
        wave: u64,
        segment: u64,
    ) -> Self {
        Pool {
            id,
            bet_unit,
            ratio,

            brokerage_ratio,
            pot_ratio,
            pot,
            base_line,
            boundary,
            suction,
            bonus,
            jackpot,
            advance,
            wave,
            segment,
        }
    }
}

impl Pool {
    pub fn draw() {}

    fn increase_pot(&mut self) -> &Self {
        self.pot += self.pot_ratio * self.bet_unit;
        self.suction += 1;
        self
    }
}
