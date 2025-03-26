use std::io::{self, BufRead};
use std::cmp::Ordering;

// Helper function to sort in descending order
fn cmp_desc(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

// Function to calculate maximum weight
fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort pizzas in descending order
    pizzas.sort_by(cmp_desc);

    let mut total: i64 = 0;
    let mut index = 0;

    // Add "odd" amount of pizzas to the total
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip one pizza
    index += 1;

    // Add "even" amount of pizzas to the total, skipping pizzas each iteration
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of pizzas
    let n: usize = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Expected number of pizzas"))??
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number format"))?;

    // Read the array of pizzas
    let mut arr: Vec<i32> = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Expected pizza array"))??
        .trim()
        .split_whitespace()
        .map(|x| x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number format")))
        .collect::<Result<_, _>>()?;

    // Calculate and print the result
    println!("{}", max_weight(&mut arr));
    Ok(())
}