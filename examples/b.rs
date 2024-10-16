use slot_algorithm::pool::Pool;

fn main() {
    let mut pool = Pool::new(
        1,
        1,
        1000,
        500,
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

    pool.draw(1, 2);
    println!("{:?}", pool);

    pool.draw(1, 5);
    println!("{:?}", pool);
}
