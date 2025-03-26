use std::collections::HashMap;
use std::f64;

fn main() {
    // Input: initialCurrency
    let mut initial_currency = String::new();
    std::io::stdin().read_line(&mut initial_currency).unwrap();
    let initial_currency = initial_currency.trim();

    // Input: pairs1 and rates1
    let mut pairs1_size = String::new();
    std::io::stdin().read_line(&mut pairs1_size).unwrap();
    let pairs1_size: usize = pairs1_size.trim().parse().unwrap();

    let mut pairs1 = Vec::new();
    let mut rates1 = Vec::new();
    for _ in 0..pairs1_size {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate: f64 = parts[2].parse().unwrap();
        pairs1.push((from, to));
        rates1.push(rate);
    }

    // Input: pairs2 and rates2
    let mut pairs2_size = String::new();
    std::io::stdin().read_line(&mut pairs2_size).unwrap();
    let pairs2_size: usize = pairs2_size.trim().parse().unwrap();

    let mut pairs2 = Vec::new();
    let mut rates2 = Vec::new();
    for _ in 0..pairs2_size {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate: f64 = parts[2].parse().unwrap();
        pairs2.push((from, to));
        rates2.push(rate);
    }

    // Call the function and get the result
    let result = max_amount(
        initial_currency,
        &pairs1,
        &rates1,
        &pairs2,
        &rates2,
    );

    // Output the result
    println!("{:.5}", result);
}

fn max_amount(
    initial_currency: &str,
    pairs1: &[(String, String)],
    rates1: &[f64],
    pairs2: &[(String, String)],
    rates2: &[f64],
) -> f64 {
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies = HashMap::new();
    let mut currency_count = 0;

    // Helper function to find or add currency index
    let mut get_currency_index = |currency: &str| -> usize {
        if let Some(&index) = currencies.get(currency) {
            index
        } else {
            let index = currency_count;
            currencies.insert(currency.to_string(), index);
            currency_count += 1;
            index
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
    for (i, (from, to)) in pairs1.iter().enumerate() {
        let from_idx = get_currency_index(from);
        let to_idx = get_currency_index(to);
        graph1[from_idx][to_idx] = rates1[i];
        graph1[to_idx][from_idx] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for (i, (from, to)) in pairs2.iter().enumerate() {
        let from_idx = get_currency_index(from);
        let to_idx = get_currency_index(to);
        graph2[from_idx][to_idx] = rates2[i];
        graph2[to_idx][from_idx] = 1.0 / rates2[i];
    }

    // Apply Floyd-Warshall algorithm on both graphs
    for k in 0..currency_count {
        for i in 0..currency_count {
            for j in 0..currency_count {
                graph1[i][j] = f64::max(graph1[i][j], graph1[i][k] * graph1[k][j]);
                graph2[i][j] = f64::max(graph2[i][j], graph2[i][k] * graph2[k][j]);
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
        max_amount = f64::max(max_amount, amount_day2);
    }

    max_amount
}