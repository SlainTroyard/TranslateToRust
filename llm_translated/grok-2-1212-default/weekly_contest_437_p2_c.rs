use std::io::{self, BufRead};

fn max_weight(pizzas: &mut [i32]) -> i64 {
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort pizzas in descending order
    pizzas.sort_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    // Add odd-indexed pizzas
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip one pizza
    index += 1;

    // Add even-indexed pizzas
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of pizzas
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read pizza weights
    let mut pizzas: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the maximum weight
    let result = max_weight(&mut pizzas);
    println!("{}", result);

    Ok(())
}