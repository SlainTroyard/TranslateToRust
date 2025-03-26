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

                if let Some(&current_from) = best.get(from) {
                    if let Some(current_to) = best.get_mut(to) {
                        *current_to = (*current_to).max(current_from * rate);
                    }
                }

                if let Some(&current_to) = best.get(to) {
                    if let Some(current_from) = best.get_mut(from) {
                        *current_from = (*current_from).max(current_to / rate);
                    }
                }
            }
        }
    }

    fn max_amount(initial_currency: String, pairs1: Vec<Vec<String>>, rates1: Vec<f64>, pairs2: Vec<Vec<String>>, rates2: Vec<f64>) -> f64 {
        let mut best = HashMap::new();
        best.insert(initial_currency.clone(), 1.0);
        
        Self::bellman(&mut best, &pairs1, &rates1);
        Self::bellman(&mut best, &pairs2, &rates2);
        
        *best.get(&initial_currency).unwrap_or(&0.0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let initial_currency = lines.next().unwrap()?.trim().to_string();
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

    let solution = Solution;
    let result = solution.max_amount(initial_currency, pairs1, rates1, pairs2, rates2);

    println!("{:.5}", result);

    Ok(())
}