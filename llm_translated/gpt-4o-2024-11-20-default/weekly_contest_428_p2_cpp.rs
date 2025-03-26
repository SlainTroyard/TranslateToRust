use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn bellman(
        best: &mut HashMap<String, f64>,
        pairs: &Vec<Vec<String>>,
        rates: &Vec<f64>,
    ) {
        for _ in 0..pairs.len() {
            for (i, pair) in pairs.iter().enumerate() {
                let rate = rates[i];
                let from = &pair[0];
                let to = &pair[1];

                if let Some(&best_from) = best.get(from) {
                    let new_to_value = best_from * rate;
                    let best_to_value = best.entry(to.clone()).or_insert(0.0);
                    if new_to_value > *best_to_value {
                        *best_to_value = new_to_value;
                    }
                }

                if let Some(&best_to) = best.get(to) {
                    let new_from_value = best_to / rate;
                    let best_from_value = best.entry(from.clone()).or_insert(0.0);
                    if new_from_value > *best_from_value {
                        *best_from_value = new_from_value;
                    }
                }
            }
        }
    }

    fn max_amount(
        initial_currency: &str,
        pairs1: Vec<Vec<String>>,
        rates1: Vec<f64>,
        pairs2: Vec<Vec<String>>,
        rates2: Vec<f64>,
    ) -> f64 {
        let mut best = HashMap::new();
        best.insert(initial_currency.to_string(), 1.0);

        Self::bellman(&mut best, &pairs1, &rates1);
        Self::bellman(&mut best, &pairs2, &rates2);

        best.get(initial_currency).copied().unwrap_or(0.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read initial currency
    let initial_currency = lines.next().unwrap().unwrap();

    // Read n1 and pairs1 with rates1
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(n1);
    let mut rates1 = Vec::with_capacity(n1);

    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();

        pairs1.push(vec![from, to]);
        rates1.push(rate);
    }

    // Read n2 and pairs2 with rates2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(n2);
    let mut rates2 = Vec::with_capacity(n2);

    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();

        pairs2.push(vec![from, to]);
        rates2.push(rate);
    }

    // Calculate result
    let result = Solution::max_amount(
        &initial_currency,
        pairs1,
        rates1,
        pairs2,
        rates2,
    );

    // Print result with 5 decimal places
    println!("{:.5}", result);
}