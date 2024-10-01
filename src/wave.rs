use rand::{seq::SliceRandom, Rng};

const GOLD_LESS: [f64; 11] = [
    0.382, 0.382, 0.5, 0.5, 0.5, 0.618, 0.618, 0.618, 0.764, 0.764, 0.764,
];
const GOLD_MORE: [f64; 3] = [1.0, 1.309, 1.618];
// const GOLD_FIVE: [f64; 4] = [0.764, 1.0, 1.309, 1.618];
const GOLD_ADJST_MORE: [f64; 3] = [1.0, 1.309, 1.618];
const GOLD_ADJST_LESS: [f64; 9] = [0.618, 0.618, 0.764, 0.764, 0.764, 1.0, 1.0, 1.0, 1.171];

#[derive(Debug, PartialEq)]
pub enum WaveState {
    Ascent,
    Fall,
}

pub fn get_state(pot: f64, wave: &Vec<f64>) -> WaveState {
    let h = wave[0];
    if pot > h {
        WaveState::Fall
    } else {
        WaveState::Ascent
    }
}

pub fn create_wave(pot: f64, baseline: f64, boundary: f64) -> Vec<f64> {
    let down = pot - baseline;
    let up = boundary - pot;
    let rand = rand::thread_rng().gen_range(0.0..(down + up));
    if rand < up {
        span_wave(pot, boundary)
    } else {
        span_wave(pot, baseline)
    }
}

fn span_wave(from: f64, to: f64) -> Vec<f64> {
    let len = to - from;
    let wave = generate_wave(len);
    wave.into_iter()
        .scan(from, |acc, x| {
            let point = *acc + x;
            *acc = point;
            Some(point)
        })
        .collect()
}

fn generate_wave(len: f64) -> Vec<f64> {
    let ratios = driving_wave(5);
    let lens = ratio_to_len(len, ratios);
    create_level_wave(lens, 3)
}

fn create_level_wave(lens: Vec<f64>, level: u32) -> Vec<f64> {
    if level == 0 {
        return lens;
    }
    let mut new_lens = Vec::new();
    for l in lens {
        let sub_wave = create_sub_wave(l);
        new_lens.extend(sub_wave);
    }
    create_level_wave(new_lens, level - 1)
}

fn create_sub_wave(len: f64) -> Vec<f64> {
    let ratios = if rand::random::<bool>() {
        driving_wave(5)
    } else {
        adjustment_wave()
    };
    ratio_to_len(len, ratios)
}

fn ratio_to_len(len: f64, ratios: Vec<f64>) -> Vec<f64> {
    let total: f64 = ratios.iter().sum();
    let base = 1.0 / total;
    ratios.into_iter().map(|r| r * base * len).collect()
}

fn driving_wave(n: usize) -> Vec<f64> {
    let coefficients = span_driving_coefficient(n);
    println!("driving coefficients: {:?}", coefficients);
    span_ratio(coefficients)
}

fn span_driving_coefficient(n: usize) -> Vec<f64> {
    let mut coefficients = Vec::new();
    // let mut last_wave = 1.0;
    for i in 1..=n {
        let ratio = if i % 2 == 1 {
            *GOLD_MORE.choose(&mut rand::thread_rng()).unwrap()
        } else {
            -*GOLD_LESS.choose(&mut rand::thread_rng()).unwrap()
            // -last_wave * *GOLD_LESS.choose(&mut rand::thread_rng()).unwrap()
        };
        // last_wave = ratio;
        coefficients.push(ratio);
    }
    coefficients
}

fn adjustment_wave() -> Vec<f64> {
    let coefficients = span_adjustment_coefficient(3);
    span_ratio(coefficients)
}

fn span_adjustment_coefficient(n: usize) -> Vec<f64> {
    let mut coefficients = Vec::new();
    for i in 1..=n {
        let ratio = if i % 2 == 1 {
            *GOLD_ADJST_MORE.choose(&mut rand::thread_rng()).unwrap()
        } else {
            -*GOLD_ADJST_LESS.choose(&mut rand::thread_rng()).unwrap()
        };
        coefficients.push(ratio);
    }
    coefficients
}

fn span_ratio(coefficients: Vec<f64>) -> Vec<f64> {
    let total: f64 = coefficients.iter().sum();
    let base = 1.0 / total;
    coefficients.into_iter().map(|r| r * base).collect()
}
