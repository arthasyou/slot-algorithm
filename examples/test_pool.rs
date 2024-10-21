use slot_algorithm::pool::Pool;

fn main() {
    let mut pool = Pool::new(1, 1, 1, 1000, 100, 100000, 10000);

    let a = pool.draw(1, 2);
    println!("{:?}", a);

    let b = pool.draw(1, 5);
    println!("{:?}", b);
}
