// Problem: Weekly Contest 437 Problem 2
use std::io::{self, BufRead};

fn max_weight(pizzas: &mut [i32]) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

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
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse n");
    
    // Read the array
    let arr_line = lines.next().unwrap()?;
    let mut arr: Vec<i32> = arr_line.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    // Calculate and print the result
    println!("{}", max_weight(&mut arr));
    
    Ok(())
}