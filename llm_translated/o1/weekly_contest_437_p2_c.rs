use std::io::{self, BufRead};

/// Sorts the pizzas array in descending order and computes the max weight
/// following the same logic as the original C implementation.
fn max_weight(pizzas: &mut [i32]) -> i64 {
    let pizzas_size = pizzas.len();
    // Number of days = pizzasSize / 4
    let day = pizzas_size / 4;
    // even = day/2, odd = (day+1)/2
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort descending (equivalent to qsort with a descending comparator).
    pizzas.sort_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    // Add the top 'odd' values
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip one element
    index += 1;

    // Add 'even' values, skipping one between each added element
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    // Read the number of pizzas (n)
    stdin
        .read_line(&mut input_line)
        .expect("Failed to read the number of pizzas");
    let n: usize = input_line
        .trim()
        .parse()
        .expect("Failed to parse the number of pizzas");

    // Prepare a vector to store the pizza weights
    let mut pizzas = Vec::with_capacity(n);

    // Read n integers from stdin, one at a time, ignoring any whitespace
    let mut count = 0;
    while count < n {
        input_line.clear();
        // Read a line and split by whitespace
        if stdin.read_line(&mut input_line).expect("Failed to read line") == 0 {
            // If no more input is available but we haven't read n pizzas, it's an error in the input format
            panic!("Not enough integers provided");
        }
        for token in input_line.split_whitespace() {
            pizzas.push(token.parse().expect("Failed to parse pizza weight"));
            count += 1;
            if count == n {
                break;
            }
        }
    }

    // Calculate and print the result
    println!("{}", max_weight(&mut pizzas));
}