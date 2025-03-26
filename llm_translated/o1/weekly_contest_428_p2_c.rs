// Translated from C to Rust for LeetCode Weekly Contest 428 Problem 2
// Requirements:
// 1. Entire file translated as a complete program, including main function and I/O handling
// 2. Algorithm logic remains exactly the same
// 3. Idiomatic Rust with proper error handling
// 4. Maintain the EXACT SAME stdin/stdout format as the original C code

use std::io::{self, Read};
use std::error::Error;

/// Computes the maximum amount of the initial currency one can get after two days,
/// given two sets of currency exchange pairs and their rates.
fn max_amount(
    initial_currency: &str,
    pairs1: &[[String; 2]],
    pairs1_size: usize,
    _pairs1_col_size: &[usize], // Column size array (unused, but preserved for logic parity)
    rates1: &[f64],
    _rates1_size: usize,
    pairs2: &[[String; 2]],
    pairs2_size: usize,
    _pairs2_col_size: &[usize], // Column size array (unused, but preserved for logic parity)
    rates2: &[f64],
    _rates2_size: usize,
) -> f64 {
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies: Vec<String> = Vec::with_capacity(MAX_CURRENCIES);
    let mut currency_count = 0;

    // Helper function to get or add a currency index
    // In the C code, this was an inner function referencing arrays;
    // here we capture them in a closure for similar effect.
    let mut get_currency_index = |curr: &str| -> usize {
        if let Some(pos) = currencies.iter().position(|x| x == curr) {
            pos
        } else {
            currencies.push(curr.to_string());
            let idx = currency_count;
            currency_count += 1;
            idx
        }
    };

    // Initialize both graphs with 1.0 on the diagonal and 0.0 elsewhere
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            if i == j {
                graph1[i][j] = 1.0;
                graph2[i][j] = 1.0;
            } else {
                graph1[i][j] = 0.0;
                graph2[i][j] = 0.0;
            }
        }
    }

    // Fill graph1 based on Day 1 pairs
    // (from -> to with rate, and to -> from with reciprocal)
    for i in 0..pairs1_size {
        let from_idx = get_currency_index(&pairs1[i][0]);
        let to_idx   = get_currency_index(&pairs1[i][1]);
        graph1[from_idx][to_idx] = rates1[i];
        graph1[to_idx][from_idx] = 1.0 / rates1[i];
    }

    // Fill graph2 based on Day 2 pairs
    // (from -> to with rate, and to -> from with reciprocal)
    for i in 0..pairs2_size {
        let from_idx = get_currency_index(&pairs2[i][0]);
        let to_idx   = get_currency_index(&pairs2[i][1]);
        graph2[from_idx][to_idx] = rates2[i];
        graph2[to_idx][from_idx] = 1.0 / rates2[i];
    }

    // Floyd-Warshall to find max exchange rate paths for both days
    for k in 0..currency_count {
        for i in 0..currency_count {
            for j in 0..currency_count {
                graph1[i][j] = f64::max(graph1[i][j], graph1[i][k] * graph1[k][j]);
                graph2[i][j] = f64::max(graph2[i][j], graph2[i][k] * graph2[k][j]);
            }
        }
    }

    // Find index of the initial currency
    let start_index = get_currency_index(initial_currency);

    // Calculate the maximum amount of initial currency recoverable after two days
    let mut max_amount = 1.0;
    for i in 0..currency_count {
        let amount_day1 = graph1[start_index][i];
        let amount_day2 = amount_day1 * graph2[i][start_index];
        max_amount = f64::max(max_amount, amount_day2);
    }

    max_amount
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input into a string, then tokenize (replicating scanf behavior)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    // Helper closures to parse tokens safely
    let mut read_string = || -> Option<String> {
        tokens.next().map(|s| s.to_string())
    };
    let mut read_usize = || -> Option<usize> {
        tokens.next().and_then(|s| s.parse::<usize>().ok())
    };
    let mut read_f64 = || -> Option<f64> {
        tokens.next().and_then(|s| s.parse::<f64>().ok())
    };

    // 1) Read initialCurrency
    let initial_currency = match read_string() {
        Some(s) => s,
        None => return Ok(()), // Or handle error if no input
    };

    // 2) Read pairs1Size
    let pairs1_size = match read_usize() {
        Some(n) => n,
        None => return Ok(()),
    };

    // 3) Read pairs1 and rates1
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    let mut rates1 = Vec::with_capacity(pairs1_size);
    let mut pairs1_col_size = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        // Each line: currency1 currency2 rate
        let c1 = match read_string() {
            Some(s) => s,
            None => return Ok(()),
        };
        let c2 = match read_string() {
            Some(s) => s,
            None => return Ok(()),
        };
        let rate = match read_f64() {
            Some(r) => r,
            None => return Ok(()),
        };
        pairs1.push([c1, c2]);
        rates1.push(rate);
        pairs1_col_size.push(2); // Each pair has 2 columns
    }

    // 4) Read pairs2Size
    let pairs2_size = match read_usize() {
        Some(n) => n,
        None => return Ok(()),
    };

    // 5) Read pairs2 and rates2
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);
    let mut pairs2_col_size = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        // Each line: currency1 currency2 rate
        let c1 = match read_string() {
            Some(s) => s,
            None => return Ok(()),
        };
        let c2 = match read_string() {
            Some(s) => s,
            None => return Ok(()),
        };
        let rate = match read_f64() {
            Some(r) => r,
            None => return Ok(()),
        };
        pairs2.push([c1, c2]);
        rates2.push(rate);
        pairs2_col_size.push(2); // Each pair has 2 columns
    }

    // 6) Call max_amount to compute the result
    let result = max_amount(
        &initial_currency,
        &pairs1,
        pairs1_size,
        &pairs1_col_size,
        &rates1,
        pairs1_size,
        &pairs2,
        pairs2_size,
        &pairs2_col_size,
        &rates2,
        pairs2_size,
    );

    // 7) Print result with 5 decimal places
    println!("{:.5}", result);

    // Rust handles memory cleanup automatically, no manual free needed.

    Ok(())
}