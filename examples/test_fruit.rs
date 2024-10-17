use slot_algorithm::{
    pool::Pool,
    slots::fruit::{self, FruitBet, FruitReward, FruitSymbol},
};

fn main() {
    let mut pool = Pool::new(1, 1, 1, 1000, 100000, 10000);

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
    // println!("{:?}", pool);
}
