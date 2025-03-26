use std::io::{self, BufRead};

/// Translates the logic of the C++ `Solution` class to a standalone Rust function.
fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    // Sort pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    // Calculate the number of "days" based on the size of the pizzas vector
    let days = pizzas.len() / 4;
    // 'odd' represents how many pizzas we pick from the start of the sorted array
    let odd = (days + 1) / 2;

    let mut ans = 0i64;
    // Add the first 'odd' pizzas
    for i in 0..odd {
        ans += pizzas[i] as i64;
    }

    // Add certain pizzas from the remaining ones according to the problem logic
    for i in 0..(days / 2) {
        ans += pizzas[odd + i * 2 + 1] as i64;
    }
    ans
}

fn main() -> io::Result<()> {
    // Read the integer 'n' from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Failed to parse integer n");

    // Read 'n' pizza weights from stdin, allowing them to be split across lines
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut pizzas = Vec::with_capacity(n);
    while pizzas.len() < n {
        if let Some(line) = lines.next() {
            let line = line?;
            // Parse integers on the current line and extend our 'pizzas' vector
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse pizza weight"))
                .collect::<Vec<_>>();
            pizzas.extend(numbers);
        } else {
            // If no more lines are available, break out of the loop
            break;
        }
    }

    // Truncate to 'n' in case we read more than needed
    pizzas.truncate(n);

    // Compute the result using our max_weight function
    let ans = max_weight(&mut pizzas);

    // Print the result to stdout
    println!("{}", ans);

    Ok(())
}