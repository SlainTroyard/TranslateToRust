use std::collections::HashMap;
use std::io::{self, BufRead};

fn max_amount(
    initial_currency: &str,
    pairs1: &[(String, String)],
    rates1: &[f64],
    pairs2: &[(String, String)],
    rates2: &[f64],
) -> f64 {
    const MAX_CURRENCIES: usize = 20;

    // Helper function to find or add currency index
    fn get_currency_index(currency: &str, currencies: &mut Vec<String>, currency_count: &mut usize) -> usize {
        if let Some(index) = currencies.iter().position(|c| c == currency) {
            index
        } else {
            currencies.push(currency.to_string());
            *currency_count += 1;
            *currency_count - 1
        }
    }

    let mut graph1 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies = Vec::new();
    let mut currency_count = 0;

    // Initialize graphs
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            graph1[i][j] = if i == j { 1.0 } else { 0.0 };
            graph2[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Fill graphs for Day 1
    for (i, (from, to)) in pairs1.iter().enumerate() {
        let from_index = get_currency_index(from, &mut currencies, &mut currency_count);
        let to_index = get_currency_index(to, &mut currencies, &mut currency_count);
        graph1[from_index][to_index] = rates1[i];
        graph1[to_index][from_index] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for (i, (from, to)) in pairs2.iter().enumerate() {
        let from_index = get_currency_index(from, &mut currencies, &mut currency_count);
        let to_index = get_currency_index(to, &mut currencies, &mut currency_count);
        graph2[from_index][to_index] = rates2[i];
        graph2[to_index][from_index] = 1.0 / rates2[i];
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
    let start_index = get_currency_index(initial_currency, &mut currencies, &mut currency_count);

    // Calculate the maximum amount
    let mut max_amount = 1.0;
    for i in 0..currency_count {
        let amount_day1 = graph1[start_index][i];
        let amount_day2 = amount_day1 * graph2[i][start_index];
        max_amount = max_amount.max(amount_day2);
    }

    max_amount
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: initialCurrency
    let initial_currency = lines.next().unwrap().unwrap();

    // Input: pairs1 and rates1
    let pairs1_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    let mut rates1 = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs1.push((from, to));
        rates1.push(rate);
    }

    // Input: pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs2.push((from, to));
        rates2.push(rate);
    }

    // Call the function and get the result
    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);

    // Output the result
    println!("{:.5}", result);
}