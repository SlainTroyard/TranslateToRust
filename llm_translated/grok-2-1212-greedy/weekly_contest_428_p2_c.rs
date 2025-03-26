use std::io::{self, BufRead};
use std::collections::HashMap;

const MAX_CURRENCIES: usize = 20;

fn max_amount(initial_currency: &str, pairs1: &[(String, String, f64)], pairs2: &[(String, String, f64)]) -> f64 {
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies = HashMap::new();
    let mut currency_count = 0;

    // Helper function to find or add currency index
    let mut get_currency_index = |currency: &str| -> usize {
        if let Some(&index) = currencies.get(currency) {
            index
        } else {
            currencies.insert(currency.to_string(), currency_count);
            currency_count += 1;
            currency_count - 1
        }
    };

    // Initialize graphs
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            if i == j {
                graph1[i][j] = 1.0;
                graph2[i][j] = 1.0;
            }
        }
    }

    // Fill graphs for Day 1
    for (from, to, rate) in pairs1 {
        let from_index = get_currency_index(from);
        let to_index = get_currency_index(to);
        graph1[from_index][to_index] = *rate;
        graph1[to_index][from_index] = 1.0 / rate;
    }

    // Fill graphs for Day 2
    for (from, to, rate) in pairs2 {
        let from_index = get_currency_index(from);
        let to_index = get_currency_index(to);
        graph2[from_index][to_index] = *rate;
        graph2[to_index][from_index] = 1.0 / rate;
    }

    // Apply Floyd-Warshall algorithm on both graphs
    for k in 0..currency_count {
        for i in 0..currency_count {
            for j in 0..currency_count {
                graph1[i][j] = graph1[i][j].max(graph1[i][k] * graph1[k][j]);
                graph2[i][j] = graph2[i][j].max(graph2[i][k] * graph2[k][j]);
            }
        }
    }

    // Find the initial currency index
    let start_index = get_currency_index(initial_currency);

    // Calculate the maximum amount
    let mut max_amount = 1.0;
    for i in 0..currency_count {
        let amount_day1 = graph1[start_index][i];
        let amount_day2 = amount_day1 * graph2[i][start_index];
        max_amount = max_amount.max(amount_day2);
    }

    max_amount
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: initialCurrency
    let initial_currency = lines.next().unwrap()?.trim().to_string();

    // Input: pairs1 and rates1
    let pairs1_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs1.push((from, to, rate));
    }

    // Input: pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs2.push((from, to, rate));
    }

    // Call the function and get the result
    let result = max_amount(&initial_currency, &pairs1, &pairs2);

    // Output the result
    println!("{:.5}", result);

    Ok(())
}