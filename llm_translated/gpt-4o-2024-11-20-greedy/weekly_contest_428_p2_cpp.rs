use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn bellman(
        best: &mut HashMap<String, f64>,
        pairs: &[Vec<String>],
        rates: &[f64],
    ) {
        for _ in 0..pairs.len() {
            for (i, pair) in pairs.iter().enumerate() {
                let rate = rates[i];
                let (from, to) = (&pair[0], &pair[1]);

                if let Some(&best_from) = best.get(from) {
                    let best_to = best.entry(to.clone()).or_insert(0.0);
                    *best_to = best_to.max(best_from * rate);
                }

                if let Some(&best_to) = best.get(to) {
                    let best_from = best.entry(from.clone()).or_insert(0.0);
                    *best_from = best_from.max(best_to / rate);
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

        *best.get(initial_currency).unwrap_or(&0.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read initial currency
    let initial_currency = lines.next().unwrap().unwrap();

    // Read number of pairs in the first set
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read pairs1 and rates1
    let mut pairs1 = Vec::new();
    let mut rates1 = Vec::new();
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs1.push(vec![from, to]);
        rates1.push(rate);
    }

    // Read number of pairs in the second set
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read pairs2 and rates2
    let mut pairs2 = Vec::new();
    let mut rates2 = Vec::new();
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs2.push(vec![from, to]);
        rates2.push(rate);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.max_amount(&initial_currency, pairs1, rates1, pairs2, rates2);

    // Print the result with fixed precision
    println!("{:.5}", result);
}