use std::collections::HashMap;
use std::io::{self, BufRead};

fn max_amount(
    initial_currency: &str,
    pairs1: Vec<(&str, &str)>,
    rates1: Vec<f64>,
    pairs2: Vec<(&str, &str)>,
    rates2: Vec<f64>,
) -> f64 {
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
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
            graph1[i][j] = if i == j { 1.0 } else { 0.0 };
            graph2[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Fill graphs for Day 1
    for (i, &(from, to)) in pairs1.iter().enumerate() {
        let from_idx = get_currency_index(from);
        let to_idx = get_currency_index(to);
        graph1[from_idx][to_idx] = rates1[i];
        graph1[to_idx][from_idx] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for (i, &(from, to)) in pairs2.iter().enumerate() {
        let from_idx = get_currency_index(from);
        let to_idx = get_currency_index(to);
        graph2[from_idx][to_idx] = rates2[i];
        graph2[to_idx][from_idx] = 1.0 / rates2[i];
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: initialCurrency
    let initial_currency = lines.next().unwrap().unwrap();

    // Input: pairs1 and rates1
    let pairs1_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs1 = Vec::new();
    let mut rates1 = Vec::new();

    for _ in 0..pairs1_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs1.push((from, to));
        rates1.push(rate);
    }

    // Input: pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2 = Vec::new();
    let mut rates2 = Vec::new();

    for _ in 0..pairs2_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs2.push((from, to));
        rates2.push(rate);
    }

    // Call the function and get the result
    let result = max_amount(&initial_currency, pairs1, rates1, pairs2, rates2);

    // Output the result
    println!("{:.5}", result);
}