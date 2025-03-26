use std::io;

/// Computes the maximum weight based on the given algorithm.
pub fn max_weight(pizzas: &mut [i32]) -> i64 {
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort the pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let mut total = 0i64;

    // Add the first 'odd' elements (largest ones)
    for &p in &pizzas[0..odd] {
        total += p as i64;
    }

    // Start at index 'odd + 1', then take every other element for 'even' times
    let mut index = odd + 1;
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    // Read the number of elements
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut pizzas: Vec<i32> = Vec::with_capacity(n);

    // Read the pizza weights
    for _ in 0..n {
        let num = tokens.next().unwrap().parse().unwrap();
        pizzas.push(num);
    }

    // Compute and print the result
    println!("{}", max_weight(pizzas.as_mut_slice()));
}