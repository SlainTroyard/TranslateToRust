use std::io;

fn main() {
    // Read the number of pizzas from stdin
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Allocate and read the pizza weights
    let mut arr = vec![0; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr[i] = input.trim().parse().expect("Please type a number!");
    }

    // Calculate and print the result
    println!("{}", max_weight(&arr));
}

/// Sorts the array in descending order and calculates the maximum weight.
fn max_weight(pizzas: &[i32]) -> i64 {
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort the slice in descending order
    let mut sorted_pizzas = pizzas.to_vec();
    sorted_pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    // Sum the first `odd` elements
    for _ in 0..odd {
        total += sorted_pizzas[index] as i64;
        index += 1;
    }

    // Skip one element
    index += 1;

    // Sum every second element starting from the current index
    for _ in 0..even {
        total += sorted_pizzas[index] as i64;
        index += 2;
    }

    total
}