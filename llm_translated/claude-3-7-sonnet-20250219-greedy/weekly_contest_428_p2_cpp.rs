use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
        for _ in 0..pairs.len() {
            for i in 0..pairs.len() {
                let from = &pairs[i][0];
                let to = &pairs[i][1];
                let rate = rates[i];
                
                // Update best rate for destination currency
                let from_value = *best.get(from).unwrap_or(&0.0);
                let to_value = *best.get(to).unwrap_or(&0.0);
                
                let new_to_value = from_value * rate;
                if new_to_value > to_value {
                    best.insert(to.clone(), new_to_value);
                }
                
                // Update best rate for source currency
                let new_from_value = to_value / rate;
                if new_from_value > from_value {
                    best.insert(from.clone(), new_from_value);
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
        
        Self::bellman(&mut best, &pairs1, &rates1);
        Self::bellman(&mut best, &pairs2, &rates2);
        
        *best.get(&initial_currency).unwrap_or(&1.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read initial currency
    let initial_currency = lines.next().unwrap().unwrap();
    
    // Read first set of pairs and rates
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(n1);
    let mut rates1 = Vec::with_capacity(n1);
    
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        pairs1.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates1.push(parts[2].parse::<f64>().unwrap());
    }
    
    // Read second set of pairs and rates
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(n2);
    let mut rates2 = Vec::with_capacity(n2);
    
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        pairs2.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates2.push(parts[2].parse::<f64>().unwrap());
    }
    
    // Calculate and print result
    let solution = Solution;
    let result = solution.max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);
}