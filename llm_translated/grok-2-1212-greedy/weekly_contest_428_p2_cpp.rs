use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

struct Solution;

impl Solution {
    fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
        for _ in 0..pairs.len() {
            for i in 0..pairs.len() {
                let from = &pairs[i][0];
                let to = &pairs[i][1];
                let rate = rates[i];

                // Update best rate for 'to' currency
                if let Some(&current_best_to) = best.get(to) {
                    let new_best_to = best[from] * rate;
                    if new_best_to > current_best_to {
                        best.insert(to.clone(), new_best_to);
                    }
                }

                // Update best rate for 'from' currency
                if let Some(&current_best_from) = best.get(from) {
                    let new_best_from = best[to] / rate;
                    if new_best_from > current_best_from {
                        best.insert(from.clone(), new_best_from);
                    }
                }
            }
        }
    }

    fn max_amount(initial_currency: String, pairs1: Vec<Vec<String>>, rates1: Vec<f64>, pairs2: Vec<Vec<String>>, rates2: Vec<f64>) -> f64 {
        let mut best = HashMap::new();
        best.insert(initial_currency.clone(), 1.0);
        Solution::bellman(&mut best, &pairs1, &rates1);
        Solution::bellman(&mut best, &pairs2, &rates2);
        *best.get(&initial_currency).unwrap_or(&1.0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read initial currency
    let initial_currency = lines.next().unwrap()?.trim().to_string();

    // Read n1 and process pairs1 and rates1
    let n1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(n1);
    let mut rates1 = Vec::with_capacity(n1);

    for _ in 0..n1 {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs1.push(vec![from, to]);
        rates1.push(rate);
    }

    // Read n2 and process pairs2 and rates2
    let n2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(n2);
    let mut rates2 = Vec::with_capacity(n2);

    for _ in 0..n2 {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        let rate: f64 = parts.next().unwrap().parse().unwrap();
        pairs2.push(vec![from, to]);
        rates2.push(rate);
    }

    // Calculate and print result
    let solution = Solution;
    let result = solution.max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);

    Ok(())
}