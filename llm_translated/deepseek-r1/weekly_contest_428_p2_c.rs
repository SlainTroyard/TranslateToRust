use std::collections::HashMap;
use std::io::{self, BufRead};

fn max_amount(
    initial_currency: &str,
    pairs1: &[[String; 2]],
    rates1: &[f64],
    pairs2: &[[String; 2]],
    rates2: &[f64],
) -> f64 {
    let mut graph1 = [[0.0; 20]; 20];
    let mut graph2 = [[0.0; 20]; 20];
    for i in 0..20 {
        graph1[i][i] = 1.0;
        graph2[i][i] = 1.0;
    }

    let mut currency_indices = HashMap::new();
    let mut currency_count = 0;

    let get_currency_index = |currency: &str, map: &mut HashMap<String, usize>, count: &mut usize| -> usize {
        *map.entry(currency.to_string()).or_insert_with(|| {
            let index = *count;
            *count += 1;
            index
        })
    };

    // Populate graph1 with day1 conversion rates
    for (pair, &rate) in pairs1.iter().zip(rates1) {
        let from = get_currency_index(&pair[0], &mut currency_indices, &mut currency_count);
        let to = get_currency_index(&pair[1], &mut currency_indices, &mut currency_count);
        graph1[from][to] = rate;
        graph1[to][from] = 1.0 / rate;
    }

    // Populate graph2 with day2 conversion rates
    for (pair, &rate) in pairs2.iter().zip(rates2) {
        let from = get_currency_index(&pair[0], &mut currency_indices, &mut currency_count);
        let to = get_currency_index(&pair[1], &mut currency_indices, &mut currency_count);
        graph2[from][to] = rate;
        graph2[to][from] = 1.0 / rate;
    }

    let current_currency_count = currency_indices.len();

    // Floyd-Warshall algorithm for both graphs to find optimal paths
    for k in 0..current_currency_count {
        for i in 0..current_currency_count {
            for j in 0..current_currency_count {
                graph1[i][j] = graph1[i][j].max(graph1[i][k] * graph1[k][j]);
                graph2[i][j] = graph2[i][j].max(graph2[i][k] * graph2[k][j]);
            }
        }
    }

    // Get start index (may add initial currency if not present)
    let start_index = get_currency_index(initial_currency, &mut currency_indices, &mut currency_count);

    // Calculate maximum amount by considering all possible intermediate currencies
    let mut max_amount = 1.0;
    for i in 0..currency_count {
        let day1_gain = graph1[start_index][i];
        let day2_gain = day1_gain * graph2[i][start_index];
        max_amount = max_amount.max(day2_gain);
    }

    max_amount
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read initial currency
    let initial_currency = lines.next().unwrap().trim().to_string();

    // Read day1 conversion pairs
    let pairs1_size: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    let mut rates1 = Vec::with_capacity(pairs1_size);
    for _ in 0..pairs1_size {
        let line = lines.next().unwrap();
        let mut parts = line.trim().split_whitespace();
        pairs1.push([
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string()
        ]);
        rates1.push(parts.next().unwrap().parse().unwrap());
    }

    // Read day2 conversion pairs
    let pairs2_size: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    let mut rates2 = Vec::with_capacity(pairs2_size);
    for _ in 0..pairs2_size {
        let line = lines.next().unwrap();
        let mut parts = line.trim().split_whitespace();
        pairs2.push([
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string()
        ]);
        rates2.push(parts.next().unwrap().parse().unwrap());
    }

    // Calculate and print result with 5 decimal places
    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);
    println!("{:.5}", result);
}