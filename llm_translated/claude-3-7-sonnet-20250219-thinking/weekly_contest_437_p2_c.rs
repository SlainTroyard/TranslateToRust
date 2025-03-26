use std::io::{self, Read};

/// Calculates the maximum weight based on the given algorithm
fn max_weight(mut pizzas: Vec<i32>) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort in descending order (equivalent to the C qsort with cmp function)
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    // Add first 'odd' elements
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }
    
    // Skip one element
    index += 1;
    
    // Add 'even' elements with gaps of 2
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }
    
    total
}

fn main() {
    // Read line containing n
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).expect("Failed to read input");
    let n: usize = n_input.trim().parse().expect("Failed to parse n");
    
    // Read line containing array elements
    let mut arr_input = String::new();
    io::stdin().read_line(&mut arr_input).expect("Failed to read input");
    
    // Parse the array elements
    let arr: Vec<i32> = arr_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    // Ensure we have the expected number of elements
    assert_eq!(arr.len(), n, "Array size doesn't match n");
    
    // Call the function and print the result
    println!("{}", max_weight(arr));
}