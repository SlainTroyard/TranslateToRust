use std::io::{self, Read};

fn max_weight(pizzas: &[i32]) -> i64 {
    let mut pizzas = pizzas.to_vec();
    // Sort pizza weights in descending order for optimal selection
    pizzas.sort_by(|a, b| b.cmp(a));
    
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;
    
    let mut total = 0i64;
    let mut index = 0;
    
    // Select largest 'odd' number of pizzas first
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }
    
    // Skip one pizza after odd selection
    index += 1;
    
    // Select every other pizza for even days to maximize weight
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }
    
    total
}

fn main() {
    // Read entire input including multiple lines
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Parse all whitespace-separated integers
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer input"));
    
    // First token is array size, remainder are elements
    let n = tokens.next().expect("No input provided") as usize;
    let pizzas: Vec<i32> = tokens.collect();
    
    // Validate input length matches specified size
    if pizzas.len() != n {
        panic!("Input length mismatch. Expected {}, got {}", n, pizzas.len());
    }
    
    // Calculate and print maximum weight using original algorithm
    println!("{}", max_weight(&pizzas));
}