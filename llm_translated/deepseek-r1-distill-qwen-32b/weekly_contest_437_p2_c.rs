use std::io;
use std::io::Read;

fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort the pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let mut total = 0;
    let mut index = 0;

    // Sum the first 'odd' pizzas
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip the next pizza
    index += 1;

    // Sum every other pizza for 'even' times
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().ok_or("No input for n")?.parse()?;

    let mut pizzas = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens.next().ok_or("Not enough pizza weights")?.parse()?;
        pizzas.push(num);
    }

    let total = max_weight(&mut pizzas);
    println!("{}", total);

    Ok(())
}