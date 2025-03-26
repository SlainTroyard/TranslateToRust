// Problem: Weekly Contest 428 Problem 2
use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

fn max_amount(
    initial_currency: &str,
    pairs1: &Vec<Vec<String>>,
    rates1: &Vec<f64>,
    pairs2: &Vec<Vec<String>>,
    rates2: &Vec<f64>,
) -> f64 {
    // Define maximum number of currencies (from constraints)
    const MAX_CURRENCIES: usize = 20;
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currency_map: HashMap<String, usize> = HashMap::new();
    let mut currency_count = 0;

    // Helper function to find or add currency index
    fn get_currency_index(currency: &str, currency_map: &mut HashMap<String, usize>, currency_count: &mut usize) -> usize {
        if let Some(&index) = currency_map.get(currency) {
            return index;
        }
        let index = *currency_count;
        currency_map.insert(currency.to_string(), index);
        *currency_count += 1;
        index
    }

    // Initialize graphs
    for i in 0..MAX_CURRENCIES {
        for j in 0..MAX_CURRENCIES {
            graph1[i][j] = if i == j { 1.0 } else { 0.0 };
            graph2[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Fill graphs for Day 1
    for i in 0..pairs1.len() {
        let from_currency = &pairs1[i][0];
        let to_currency = &pairs1[i][1];
        let rate1 = rates1[i];
        let from = get_currency_index(from_currency, &mut currency_map, &mut currency_count);
        let to = get_currency_index(to_currency, &mut currency_map, &mut currency_count);
        graph1[from][to] = rate1;
        graph1[to][from] = 1.0 / rate1;
    }

    // Fill graphs for Day 2
    for i in 0..pairs2.len() {
        let from_currency = &pairs2[i][0];
        let to_currency = &pairs2[i][1];
        let rate2 = rates2[i];
        let from = get_currency_index(from_currency, &mut currency_map, &mut currency_count);
        let to = get_currency_index(to_currency, &mut currency_map, &mut currency_count);
        graph2[from][to] = rate2;
        graph2[to][from] = 1.0 / rate2;
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
    let start_index = get_currency_index(initial_currency, &mut currency_map, &mut currency_count);

    // Calculate the maximum amount
    let mut max_amount = 1.0;
    for i in 0..currency_count {
        let amount_day1 = graph1[start_index][i];
        let amount_day2 = amount_day1 * graph2[i][start_index];
        max_amount = f64::max(max_amount, amount_day2);
    }

    max_amount
}

fn main() {
    // Input: initialCurrency
    let mut initial_currency = String::new();
    io::stdin().read_line(&mut initial_currency).unwrap();
    let initial_currency = initial_currency.trim();

    // Input: pairs1 and rates1
    let mut pairs1_size_str = String::new();
    io::stdin().read_line(&mut pairs1_size_str).unwrap();
    let pairs1_size: usize = pairs1_size_str.trim().parse().unwrap();

    let mut pairs1: Vec<Vec<String>> = Vec::with_capacity(pairs1_size);
    let mut rates1: Vec<f64> = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs1.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates1.push(parts[2].parse().unwrap());
    }

    // Input: pairs2 and rates2
    let mut pairs2_size_str = String::new();
    io::stdin().read_line(&mut pairs2_size_str).unwrap();
    let pairs2_size: usize = pairs2_size_str.trim().parse().unwrap();

    let mut pairs2: Vec<Vec<String>> = Vec::with_capacity(pairs2_size);
    let mut rates2: Vec<f64> = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs2.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates2.push(parts[2].parse().unwrap());
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