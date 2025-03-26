use std::io;

fn main() {
    // Read the number of pizzas from stdin
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Read the weights of the pizzas
    let mut arr = vec![0; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr[i] = input.trim().parse().expect("Please type a number!");
    }

    // Calculate and print the maximum weight
    println!("{}", max_weight(&arr));
}

/// Compares two integers in descending order.
fn cmp(a: &i32, b: &i32) -> std::cmp::Ordering {
    b.cmp(a)
}

/// Calculates the maximum weight of pizzas that can be eaten.
///
/// # Arguments
///
/// * `pizzas` - A slice of pizza weights.
///
/// # Returns
///
/// * The maximum weight of pizzas that can be eaten.
fn max_weight(pizzas: &[i32]) -> i64 {
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort the pizzas in descending order
    let mut sorted_pizzas = pizzas.to_vec();
    sorted_pizzas.sort_by(cmp);

    let mut total: i64 = 0;
    let mut index = 0;

    // Add the weights of the first `odd` pizzas
    for _ in 0..odd {
        total += sorted_pizzas[index] as i64;
        index += 1;
    }

    // Skip one pizza
    index += 1;

    // Add the weights of every second pizza starting from the current index
    for _ in 0..even {
        total += sorted_pizzas[index] as i64;
        index += 2;
    }

    total
}