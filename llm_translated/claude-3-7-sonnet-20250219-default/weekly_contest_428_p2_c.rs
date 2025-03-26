use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn max_amount(
    initial_currency: &str,
    pairs1: &[Vec<String>],
    rates1: &[f64],
    pairs2: &[Vec<String>],
    rates2: &[f64],
) -> f64 {
    // Define maximum number of currencies (from constraints)
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = [[1.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[1.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies = Vec::with_capacity(MAX_CURRENCIES);
    let mut currency_map = HashMap::new();

    // Helper function to find or add currency index
    let mut get_currency_index = |currency: &str| -> usize {
        if let Some(&index) = currency_map.get(currency) {
            index
        } else {
            let index = currencies.len();
            currencies.push(currency.to_string());
            currency_map.insert(currency.to_string(), index);
            index
        }
    };

    // Initialize graphs with default values
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            graph1[i][j] = if i == j { 1.0 } else { 0.0 };
            graph2[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Fill graphs for Day 1
    for (i, pair) in pairs1.iter().enumerate() {
        let from = get_currency_index(&pair[0]);
        let to = get_currency_index(&pair[1]);
        graph1[from][to] = rates1[i];
        graph1[to][from] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for (i, pair) in pairs2.iter().enumerate() {
        let from = get_currency_index(&pair[0]);
        let to = get_currency_index(&pair[1]);
        graph2[from][to] = rates2[i];
        graph2[to][from] = 1.0 / rates2[i];
    }

    let currency_count = currencies.len();

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
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    let mut rates1 = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let from_currency = parts[0].to_string();
        let to_currency = parts[1].to_string();
        let rate: f64 = parts[2].parse().unwrap();
        
        pairs1.push(vec![from_currency, to_currency]);
        rates1.push(rate);
    }

    // Input: pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let from_currency = parts[0].to_string();
        let to_currency = parts[1].to_string();
        let rate: f64 = parts[2].parse().unwrap();
        
        pairs2.push(vec![from_currency, to_currency]);
        rates2.push(rate);
    }

    // Call the function and get the result
    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);

    // Output the result with exactly 5 decimal places
    println!("{:.5}", result);
}