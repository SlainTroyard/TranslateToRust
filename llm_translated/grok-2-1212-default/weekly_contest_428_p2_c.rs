use std::io::{self, BufRead};

const MAX_CURRENCIES: usize = 20;

fn max_amount(initial_currency: &str, pairs1: &[Vec<String>], rates1: &[f64], pairs2: &[Vec<String>], rates2: &[f64]) -> f64 {
    let mut graph1 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = [[0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut currencies = Vec::with_capacity(MAX_CURRENCIES);

    // Helper function to find or add currency index
    fn get_currency_index(currency: &str, currencies: &mut Vec<String>) -> usize {
        if let Some(index) = currencies.iter().position(|c| c == currency) {
            index
        } else {
            currencies.push(currency.to_string());
            currencies.len() - 1
        }
    }

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
    for (i, pair) in pairs1.iter().enumerate() {
        let from = get_currency_index(&pair[0], &mut currencies);
        let to = get_currency_index(&pair[1], &mut currencies);
        graph1[from][to] = rates1[i];
        graph1[to][from] = 1.0 / rates1[i];
    }

    // Fill graphs for Day 2
    for (i, pair) in pairs2.iter().enumerate() {
        let from = get_currency_index(&pair[0], &mut currencies);
        let to = get_currency_index(&pair[1], &mut currencies);
        graph2[from][to] = rates2[i];
        graph2[to][from] = 1.0 / rates2[i];
    }

    // Apply Floyd-Warshall algorithm on both graphs
    let currency_count = currencies.len();
    for k in 0..currency_count {
        for i in 0..currency_count {
            for j in 0..currency_count {
                graph1[i][j] = graph1[i][j].max(graph1[i][k] * graph1[k][j]);
                graph2[i][j] = graph2[i][j].max(graph2[i][k] * graph2[k][j]);
            }
        }
    }

    // Find the initial currency index
    let start_index = get_currency_index(initial_currency, &mut currencies);

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
    let mut rates1 = Vec::with_capacity(pairs1_size);

    for _ in 0..pairs1_size {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        pairs1.push(vec![parts.next().unwrap().to_string(), parts.next().unwrap().to_string()]);
        rates1.push(parts.next().unwrap().parse().unwrap());
    }

    // Input: pairs2 and rates2
    let pairs2_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);

    for _ in 0..pairs2_size {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        pairs2.push(vec![parts.next().unwrap().to_string(), parts.next().unwrap().to_string()]);
        rates2.push(parts.next().unwrap().parse().unwrap());
    }

    // Call the function and get the result
    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);

    // Output the result
    println!("{:.5}", result);

    Ok(())
}