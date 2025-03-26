use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX_CURRENCIES: usize = 20;

struct CurrencyManager {
    currencies: HashMap<String, usize>,
    count: usize,
}

impl CurrencyManager {
    fn new() -> Self {
        Self {
            currencies: HashMap::new(),
            count: 0,
        }
    }

    fn get_index(&mut self, currency: String) -> usize {
        if let Some(&idx) = self.currencies.get(&currency) {
            idx
        } else {
            let idx = self.count;
            self.currencies.insert(currency, idx);
            self.count += 1;
            idx
        }
    }
}

fn max_amount(
    initial_currency: String,
    pairs1: Vec<Vec<String>>,
    rates1: Vec<f64>,
    pairs2: Vec<Vec<String>>,
    rates2: Vec<f64>,
) -> f64 {
    let mut cm = CurrencyManager::new();
    let start_idx = cm.get_index(initial_currency.clone());

    // Initialize graphs
    let mut graph1 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    let mut graph2 = vec![vec![0.0; MAX_CURRENCIES]; MAX_CURRENCIES];
    for i in 0..MAX_CURRENCIES {
        graph1[i][i] = 1.0;
        graph2[i][i] = 1.0;
    }

    // Fill graph1
    for (pair, rate) in pairs1.iter().zip(rates1.iter()) {
        let from = cm.get_index(pair[0].clone());
        let to = cm.get_index(pair[1].clone());
        graph1[from][to] = *rate;
        graph1[to][from] = 1.0 / *rate;
    }

    // Fill graph2
    for (pair, rate) in pairs2.iter().zip(rates2.iter()) {
        let from = cm.get_index(pair[0].clone());
        let to = cm.get_index(pair[1].clone());
        graph2[from][to] = *rate;
        graph2[to][from] = 1.0 / *rate;
    }

    // Floyd-Warshall algorithm for both graphs
    for k in 0..cm.count {
        for i in 0..cm.count {
            for j in 0..cm.count {
                let via_k = graph1[i][k] * graph1[k][j];
                if via_k > graph1[i][j] {
                    graph1[i][j] = via_k;
                }
                let via_k2 = graph2[i][k] * graph2[k][j];
                if via_k2 > graph2[i][j] {
                    graph2[i][j] = via_k2;
                }
            }
        }
    }

    // Calculate the maximum amount
    let mut max_amount = 1.0;
    for i in 0..cm.count {
        let amount_day1 = graph1[start_idx][i];
        let amount_day2 = amount_day1 * graph2[i][start_idx];
        if amount_day2 > max_amount {
            max_amount = amount_day2;
        }
    }

    max_amount
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read initial currency
    let initial_currency = lines.next().unwrap().unwrap().trim().to_string();

    // Read pairs1
    let pairs1_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    let mut rates1 = Vec::with_capacity(pairs1_size);
    for _ in 0..pairs1_size {
        let line = lines.next().unwrap().unwrap().trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate: f64 = parts[2].parse().unwrap();
        pairs1.push(vec![from, to]);
        rates1.push(rate);
    }

    // Read pairs2
    let pairs2_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);
    for _ in 0..pairs2_size {
        let line = lines.next().unwrap().unwrap().trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate: f64 = parts[2].parse().unwrap();
        pairs2.push(vec![from, to]);
        rates2.push(rate);
    }

    // Calculate and print the result
    let result = max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);
}