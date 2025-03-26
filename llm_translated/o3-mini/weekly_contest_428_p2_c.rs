use std::io::{self, BufRead};

const MAX_CURRENCIES: usize = 20;

fn main() -> io::Result<()> {
    // Prepare standard input reader, using locked stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Helper closure to get next non-empty token from a string iterator
    let mut get_tokens = |line: &str| -> Vec<String> {
        line.split_whitespace().map(|s| s.to_string()).collect()
    };

    // Read initial currency (a string)
    let initial_currency = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                break line.trim().to_string();
            }
        } else {
            panic!("Unexpected end of input while reading initial currency");
        }
    };

    // Read pairs1Size (an integer)
    let pairs1_size: usize = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                let tokens = get_tokens(&line);
                if let Some(token) = tokens.get(0) {
                    break token.parse().expect("Failed to parse pairs1 size");
                }
            }
        } else {
            panic!("Unexpected end of input while reading pairs1 size");
        }
    };

    // Read pairs1 and rates1.
    // Each line contains: <currency_from> <currency_to> <rate>
    let mut pairs1: Vec<(String, String)> = Vec::with_capacity(pairs1_size);
    let mut rates1: Vec<f64> = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        let tokens: Vec<String> = loop {
            if let Some(line) = lines.next() {
                let line = line?;
                let tokens = get_tokens(&line);
                if !tokens.is_empty() {
                    break tokens;
                }
            } else {
                panic!("Unexpected end of input while reading pairs1 info");
            }
        };
        if tokens.len() < 3 {
            panic!("Expected at least 3 tokens per line for pairs1");
        }
        let curr_from = tokens[0].clone();
        let curr_to = tokens[1].clone();
        let rate: f64 = tokens[2].parse().expect("Failed to parse rate for pairs1");
        pairs1.push((curr_from, curr_to));
        rates1.push(rate);
    }

    // Read pairs2Size (an integer)
    let pairs2_size: usize = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                let tokens = get_tokens(&line);
                if let Some(token) = tokens.get(0) {
                    break token.parse().expect("Failed to parse pairs2 size");
                }
            }
        } else {
            panic!("Unexpected end of input while reading pairs2 size");
        }
    };

    // Read pairs2 and rates2.
    // Each line contains: <currency_from> <currency_to> <rate>
    let mut pairs2: Vec<(String, String)> = Vec::with_capacity(pairs2_size);
    let mut rates2: Vec<f64> = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        let tokens: Vec<String> = loop {
            if let Some(line) = lines.next() {
                let line = line?;
                let tokens = get_tokens(&line);
                if !tokens.is_empty() {
                    break tokens;
                }
            } else {
                panic!("Unexpected end of input while reading pairs2 info");
            }
        };
        if tokens.len() < 3 {
            panic!("Expected at least 3 tokens per line for pairs2");
        }
        let curr_from = tokens[0].clone();
        let curr_to = tokens[1].clone();
        let rate: f64 = tokens[2].parse().expect("Failed to parse rate for pairs2");
        pairs2.push((curr_from, curr_to));
        rates2.push(rate);
    }

    // Now, compute the maximum amount using the same algorithm as the C code.
    let result = max_amount(
        &initial_currency,
        &pairs1,
        &rates1,
        &pairs2,
        &rates2,
    );
    
    // Print the result with exactly 5 decimal places
    println!("{:.5}", result);

    Ok(())
}

/// Computes the maximum amount obtainable after two days of currency exchanges.
/// This function mirrors the logic of the provided C solution.
fn max_amount(
    initial_currency: &str,
    pairs1: &[(String, String)],
    rates1: &[f64],
    pairs2: &[(String, String)],
    rates2: &[f64],
) -> f64 {
    // Initialize 2D arrays for Day 1 and Day 2 graphs.
    let mut graph1 = [[0.0f64; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0f64; MAX_CURRENCIES]; MAX_CURRENCIES];

    // Initialize the graphs: set diagonal to 1.0, others to 0.0.
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

    // A vector to hold all encountered currencies (as Strings)
    let mut currencies: Vec<String> = Vec::with_capacity(MAX_CURRENCIES);

    // Helper closure: Given a currency &str,
    // returns its index in the currencies vector. If not found, adds it.
    let mut get_currency_index = |currency: &str| -> usize {
        if let Some(pos) = currencies.iter().position(|c| c == currency) {
            pos
        } else {
            // Add currency to list and return its index
            currencies.push(currency.to_string());
            currencies.len() - 1
        }
    };

    // Fill graph1 for Day 1 using pairs1 and rates1.
    for (i, (from, to)) in pairs1.iter().enumerate() {
        let from_idx = get_currency_index(from);
        let to_idx = get_currency_index(to);
        graph1[from_idx][to_idx] = rates1[i];
        graph1[to_idx][from_idx] = 1.0 / rates1[i];
    }

    // Fill graph2 for Day 2 using pairs2 and rates2.
    for (i, (from, to)) in pairs2.iter().enumerate() {
        let from_idx = get_currency_index(from);
        let to_idx = get_currency_index(to);
        graph2[from_idx][to_idx] = rates2[i];
        graph2[to_idx][from_idx] = 1.0 / rates2[i];
    }

    // Apply the Floyd-Warshall algorithm on both graphs.
    let currency_count = currencies.len();
    for k in 0..currency_count {
        for i in 0..currency_count {
            for j in 0..currency_count {
                graph1[i][j] = f64::max(graph1[i][j], graph1[i][k] * graph1[k][j]);
                graph2[i][j] = f64::max(graph2[i][j], graph2[i][k] * graph2[k][j]);
            }
        }
    }

    // Find the index for the initial currency.
    // This will add the currency if not already present, which is consistent with the C code.
    let start_index = get_currency_index(initial_currency);

    // Calculate the maximum amount achievable
    let mut max_amount = 1.0;
    for i in 0..currencies.len() {
        let amount_day1 = graph1[start_index][i];
        let amount_day2 = amount_day1 * graph2[i][start_index];
        max_amount = f64::max(max_amount, amount_day2);
    }

    max_amount
}