use std::collections::HashMap;
use std::io::{self, BufRead};

fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
    for _ in 0..pairs.len() {
        for i in 0..pairs.len() {
            let from = &pairs[i][0];
            let to = &pairs[i][1];
            let rate = rates[i];

            if let Some(&from_value) = best.get(from) {
                let new_to_value = from_value * rate;
                let to_value = best.entry(to.to_string()).or_insert(new_to_value);
                *to_value = (*to_value).max(new_to_value);
            }

            if let Some(&to_value) = best.get(to) {
                let new_from_value = to_value / rate;
                let from_value = best.entry(from.to_string()).or_insert(new_from_value);
                *from_value = (*from_value).max(new_from_value);
            }
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
    let mut best = HashMap::new();
    best.insert(initial_currency.clone(), 1.0);

    bellman(&mut best, &pairs1, &rates1);
    bellman(&mut best, &pairs2, &rates2);

    *best.get(&initial_currency).unwrap_or(&0.0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let initial_currency = lines.next().unwrap().unwrap();

    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(n1);
    let mut rates1 = Vec::with_capacity(n1);

    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        pairs1.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates1.push(parts[2].parse().unwrap());
    }

    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(n2);
    let mut rates2 = Vec::with_capacity(n2);

    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        pairs2.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates2.push(parts[2].parse().unwrap());
    }

    let result = max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);
}