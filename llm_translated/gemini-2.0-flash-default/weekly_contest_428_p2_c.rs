use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

const MAX_CURRENCIES: usize = 20;

fn max_amount(
    initial_currency: &str,
    pairs1: &Vec<Vec<String>>,
    rates1: &Vec<f64>,
    pairs2: &Vec<Vec<String>>,
    rates2: &Vec<f64>,
) -> f64 {
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currency_indices: HashMap<String, usize> = HashMap::new();
    let mut currency_count = 0;

    // Helper function to find or add currency index
    let mut get_currency_index = |currency: &str| -> usize {
        if let Some(&index) = currency_indices.get(currency) {
            return index;
        }
        let index = currency_count;
        currency_indices.insert(currency.to_string(), index);
        currency_count += 1;
        index
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: initialCurrency
    let initial_currency = lines.next().unwrap().unwrap();

    // Input: pairs1 and rates1
    let pairs1_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut pairs1: Vec<Vec<String>> = Vec::new();
    let mut rates1: Vec<f64> = Vec::new();

    for _ in 0..pairs1_size {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<&str> = line.split_whitespace().collect();
        pairs1.push(vec![
            values[0].to_string(),
            values[1].to_string(),
        ]);
        rates1.push(values[2].parse().unwrap());
    }

    // Input: pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut pairs2: Vec<Vec<String>> = Vec::new();
    let mut rates2: Vec<f64> = Vec::new();

    for _ in 0..pairs2_size {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<&str> = line.split_whitespace().collect();
        pairs2.push(vec![
            values[0].to_string(),
            values[1].to_string(),
        ]);
        rates2.push(values[2].parse().unwrap());
    }

    // Call the function and get the result
    let result = max_amount(
        &initial_currency,
        &pairs1,
        &rates1,
        &pairs2,
        &rates2,
    );

    // Output the result
    println!("{:.5}", result);
}