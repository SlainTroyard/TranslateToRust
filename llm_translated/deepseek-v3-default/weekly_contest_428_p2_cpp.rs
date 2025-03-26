use std::collections::HashMap;
use std::io;

fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
    for _ in 0..pairs.len() {
        for i in 0..pairs.len() {
            let from = &pairs[i][0];
            let to = &pairs[i][1];
            let rate = rates[i];
            if let Some(&from_value) = best.get(from) {
                let new_to_value = from_value * rate;
                let current_to_value = best.entry(to.to_string()).or_insert(0.0);
                *current_to_value = current_to_value.max(new_to_value);
            }
            if let Some(&to_value) = best.get(to) {
                let new_from_value = to_value / rate;
                let current_from_value = best.entry(from.to_string()).or_insert(0.0);
                *current_from_value = current_from_value.max(new_from_value);
            }
        }
    }
}

fn max_amount(initial_currency: &str, pairs1: &Vec<Vec<String>>, rates1: &Vec<f64>, pairs2: &Vec<Vec<String>>, rates2: &Vec<f64>) -> f64 {
    let mut best = HashMap::new();
    best.insert(initial_currency.to_string(), 1.0);
    bellman(&mut best, pairs1, rates1);
    bellman(&mut best, pairs2, rates2);
    *best.get(initial_currency).unwrap_or(&0.0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let initial_currency = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n1: usize = input.trim().parse().expect("Invalid input");

    let mut pairs1 = Vec::new();
    let mut rates1 = Vec::new();
    for _ in 0..n1 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate: f64 = parts[2].parse().expect("Invalid rate");
        pairs1.push(vec![from, to]);
        rates1.push(rate);
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n2: usize = input.trim().parse().expect("Invalid input");

    let mut pairs2 = Vec::new();
    let mut rates2 = Vec::new();
    for _ in 0..n2 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        let rate: f64 = parts[2].parse().expect("Invalid rate");
        pairs2.push(vec![from, to]);
        rates2.push(rate);
    }

    let result = max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);
    println!("{:.5}", result);
}