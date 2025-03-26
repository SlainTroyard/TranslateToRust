use std::collections::HashMap;
use std::io;

fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
    let len = pairs.len();
    for _ in 0..len {
        for i in 0..len {
            let pair = &pairs[i];
            let a = &pair[0];
            let b = &pair[1];
            let rate = rates[i];

            let current_a = best.get(a).copied().unwrap_or(0.0);
            let current_b = best.get(b).copied().unwrap_or(0.0);

            // Update b based on a
            let new_b = current_a * rate;
            if new_b > current_b {
                best.insert(b.clone(), new_b);
            }

            // Update a based on b
            let new_a = current_b / rate;
            if new_a > current_a {
                best.insert(a.clone(), new_a);
            }
        }
    }
}

fn max_amount(initial_currency: String, pairs1: Vec<Vec<String>>, rates1: Vec<f64>, pairs2: Vec<Vec<String>>, rates2: Vec<f64>) -> f64 {
    let mut best = HashMap::new();
    best.insert(initial_currency.clone(), 1.0);

    bellman(&mut best, &pairs1, &rates1);
    bellman(&mut best, &pairs2, &rates2);

    *best.get(&initial_currency).unwrap_or(&0.0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut pos = 0;

    let initial_currency = tokens[pos].to_string();
    pos += 1;

    let n1: usize = tokens[pos].parse().unwrap();
    pos += 1;

    let mut pairs1 = Vec::new();
    let mut rates1 = Vec::new();
    for _ in 0..n1 {
        let a = tokens[pos].to_string();
        let b = tokens[pos + 1].to_string();
        let rate: f64 = tokens[pos + 2].parse().unwrap();
        pairs1.push(vec![a, b]);
        rates1.push(rate);
        pos += 3;
    }

    let n2: usize = tokens[pos].parse().unwrap();
    pos += 1;

    let mut pairs2 = Vec::new();
    let mut rates2 = Vec::new();
    for _ in 0..n2 {
        let a = tokens[pos].to_string();
        let b = tokens[pos + 1].to_string();
        let rate: f64 = tokens[pos + 2].parse().unwrap();
        pairs2.push(vec![a, b]);
        rates2.push(rate);
        pos += 3;
    }

    let result = max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);
}