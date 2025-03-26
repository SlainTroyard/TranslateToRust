use std::io;
use std::io::Read;

fn max_weight(pizzas: &mut [i32]) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    pizzas.sort_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    index += 1;
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }
    total
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", max_weight(&mut arr.clone()));

    Ok(())
}