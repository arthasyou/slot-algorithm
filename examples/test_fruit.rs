use slot_algorithm::{
    pool::Pool,
    slots::fruit::{self, FruitBet, FruitReward, FruitSymbol},
};

fn main() {
    let mut pool = Pool::new(
        1,
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

    let fruits = vec![
        FruitBet {
            symbol: FruitSymbol::Bar,
            value: 1,
        },
        FruitBet {
            symbol: FruitSymbol::LuckySeven,
            value: 1,
        },
        FruitBet {
            symbol: FruitSymbol::Apple,
            value: 1,
        },
    ];

    let rewards: Vec<FruitReward> = fruit::draw(fruits, &mut pool);
    // let a = pool.draw(1, 2);
    println!("{:?}", rewards);

    // let b = pool.draw(1, 5);
    // println!("{:?}", b);
}
