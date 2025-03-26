use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read initial currency
    let initial_currency = lines.next().unwrap().trim().to_string();

    // Read pairs1
    let pairs1_size: usize = lines.next().unwrap().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(pairs1_size);
    for _ in 0..pairs1_size {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate = parts[2].parse::<f64>().unwrap();
        pairs1.push((from, to, rate));
    }

    // Read pairs2
    let pairs2_size: usize = lines.next().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(pairs2_size);
    for _ in 0..pairs2_size {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate = parts[2].parse::<f64>().unwrap();
        pairs2.push((from, to, rate));
    }

    // Collect currencies
    let mut currencies = Vec::new();
    let mut currency_map: HashMap<String, usize> = HashMap::new();

    // Closure to add currencies and track indices
    let add_currency = |s: &str| -> usize {
        if let Some(&idx) = currency_map.get(s) {
            idx
        } else {
            let idx = currencies.len();
            currencies.push(s.to_string());
            currency_map.insert(s.to_string(), idx);
            idx
        }
    };

    // Add currencies from pairs1
    for (from, to, _) in &pairs1 {
        add_currency(from);
        add_currency(to);
    }

    // Add currencies from pairs2
    for (from, to, _) in &pairs2 {
        add_currency(from);
        add_currency(to);
    }

    let n = currencies.len();

    // Initialize graphs with 1 on diagonal and 0 elsewhere
    let mut graph1 = vec![vec![0.0; n]; n];
    let mut graph2 = vec![vec![0.0; n]; n];
    for i in 0..n {
        graph1[i][i] = 1.0;
        graph2[i][i] = 1.0;
    }

    // Fill graph1 with pairs1 rates
    for (from_str, to_str, rate) in &pairs1 {
        let from = currency_map[from_str];
        let to = currency_map[to_str];
        graph1[from][to] = *rate;
        graph1[to][from] = 1.0 / rate;
    }

    // Fill graph2 with pairs2 rates
    for (from_str, to_str, rate) in &pairs2 {
        let from = currency_map[from_str];
        let to = currency_map[to_str];
        graph2[from][to] = *rate;
        graph2[to][from] = 1.0 / rate;
    }

    // Floyd-Warshall algorithm for both graphs
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let temp1 = graph1[i][k] * graph1[k][j];
                if temp1 > graph1[i][j] {
                    graph1[i][j] = temp1;
                }
                let temp2 = graph2[i][k] * graph2[k][j];
                if temp2 > graph2[i][j] {
                    graph2[i][j] = temp2;
                }
            }
        }
    }

    // Find the index of the initial currency
    let start_index = *currency_map.get(&initial_currency).unwrap();

    // Calculate the maximum amount achievable
    let mut max_amount = 1.0;
    for i in 0..n {
        let amount_day1 = graph1[start_index][i];
        let amount_day2 = amount_day1 * graph2[i][start_index];
        if amount_day2 > max_amount {
            max_amount = amount_day2;
        }
    }

    // Output with 5 decimal places
    println!("{:.5}", max_amount);
}