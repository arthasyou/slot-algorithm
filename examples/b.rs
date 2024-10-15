use slot_algorithm::pool::{Pool, WaveState};

fn main() {
    let mut pool = Pool::new(
        1,
        1,
        1000,
        500,
        0,
        9500,
        5000000,
        0,
        8000000,
        0,
        0,
        0,
        0,
        vec![],
        (0, 1000000000),
    );

    pool.draw(1, 2, WaveState::Ascent);
    println!("{:?}", pool);

    pool.draw(1, 50, WaveState::Fall);
    println!("{:?}", pool);
}
