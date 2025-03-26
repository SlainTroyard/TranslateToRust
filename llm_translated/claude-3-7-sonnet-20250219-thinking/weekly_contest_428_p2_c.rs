use std::collections::HashMap;
use std::io::{self, BufRead};
use std::cmp::Ordering;

fn max_amount(
    initial_currency: &str,
    pairs1: &Vec<Vec<String>>,
    rates1: &Vec<f64>,
    pairs2: &Vec<Vec<String>>,
    rates2: &Vec<f64>,
) -> f64 {
    // Define maximum number of currencies (from constraints)
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies = vec![""; MAX_CURRENCIES];
    let mut currency_map = HashMap::new();
    let mut currency_count = 0;

    // Helper function to find or add currency index
    let mut get_currency_index = |currency: &str| -> usize {
        if let Some(&idx) = currency_map.get(currency) {
            return idx;
        }
        currencies[currency_count] = currency;
        currency_map.insert(currency, currency_count);
        let idx = currency_count;
        currency_count += 1;
        idx
    };

    // Initialize graphs
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            graph1[i][j] = if i == j { 1.0 } else { 0.0 };
            graph2[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Fill graphs for Day 1
    for i in 0..pairs1.len() {
        let from = get_currency_index(&pairs1[i][0]);
        let to = get_currency_index(&pairs1[i][1]);
        graph1[from][to] = rates1[i];
        graph1[to][from] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for i in 0..pairs2.len() {
        let from = get_currency_index(&pairs2[i][0]);
        let to = get_currency_index(&pairs2[i][1]);
        graph2[from][to] = rates2[i];
        graph2[to][from] = 1.0 / rates2[i];
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
    
    // Read the initial currency
    let initial_currency = lines.next().unwrap()?.trim().to_string();
    
    // Read pairs1 and rates1
    let pairs1_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    let mut rates1 = Vec::with_capacity(pairs1_size);
    
    for _ in 0..pairs1_size {
        let line = lines.next().unwrap()?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs1.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates1.push(parts[2].parse().unwrap());
    }
    
    // Read pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);
    
    for _ in 0..pairs2_size {
        let line = lines.next().unwrap()?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs2.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates2.push(parts[2].parse().unwrap());
    }
    
    // Call the function and get the result
    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);
    
    // Output the result with exactly 5 decimal places
    println!("{:.5}", result);
    
    Ok(())
}