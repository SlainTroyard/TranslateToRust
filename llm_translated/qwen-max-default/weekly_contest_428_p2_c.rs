use std::collections::HashMap;
use std::io::{self, BufRead, Write};

fn main() {
    // Read initial currency from stdin
    let mut initial_currency = String::new();
    io::stdin().read_line(&mut initial_currency).unwrap();
    let initial_currency = initial_currency.trim();

    // Read pairs and rates for Day 1
    let (pairs1, rates1) = read_pairs_and_rates();

    // Read pairs and rates for Day 2
    let (pairs2, rates2) = read_pairs_and_rates();

    // Call the function and get the result
    let result = max_amount(initial_currency, &pairs1, &rates1, &pairs2, &rates2);

    // Output the result
    println!("{:.5}", result);
}

fn read_pairs_and_rates() -> (Vec<(&str, &str)>, Vec<f64>) {
    let mut pairs = Vec::new();
    let mut rates = Vec::new();
    let stdin = io::stdin();
    let mut size = String::new();
    stdin.lock().read_line(&mut size).unwrap();
    let size: usize = size.trim().parse().unwrap();

    for _ in 0..size {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs.push((from, to));
        rates.push(rate);
    }

    (pairs, rates)
}

fn max_amount(
    initial_currency: &str,
    pairs1: &[(impl AsRef<str>, impl AsRef<str>)],
    rates1: &[f64],
    pairs2: &[(impl AsRef<str>, impl AsRef<str>)],
    rates2: &[f64],
) -> f64 {
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies: HashMap<&str, usize> = HashMap::new();
    let mut currency_count = 0;

    // Helper function to find or add currency index
    fn get_currency_index(currency: &str, currencies: &mut HashMap<&str, usize>, count: &mut usize) -> usize {
        *currencies.entry(currency).or_insert_with(|| {
            let index = *count;
            *count += 1;
            index
        })
    }

    // Initialize graphs
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            graph1[i][j] = if i == j { 1.0 } else { 0.0 };
            graph2[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Fill graphs for Day 1
    for (i, (from, to)) in pairs1.iter().enumerate() {
        let from = get_currency_index(from.as_ref(), &mut currencies, &mut currency_count);
        let to = get_currency_index(to.as_ref(), &mut currencies, &mut currency_count);
        graph1[from][to] = rates1[i];
        graph1[to][from] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for (i, (from, to)) in pairs2.iter().enumerate() {
        let from = get_currency_index(from.as_ref(), &mut currencies, &mut currency_count);
        let to = get_currency_index(to.as_ref(), &mut currencies, &mut currency_count);
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