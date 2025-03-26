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
            for i in 0..pairs.len() {
                let pair0 = &pairs[i][0];
                let pair1 = &pairs[i][1];
                let rate = rates[i];
                
                // Get current best values, defaulting to 0.0 if not present
                let best0 = *best.get(pair0).unwrap_or(&0.0);
                let best1 = *best.get(pair1).unwrap_or(&0.0);
                
                // Update best values
                best.insert(pair1.clone(), best1.max(best0 * rate));
                best.insert(pair0.clone(), best0.max(best1 / rate));
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
        
        *best.get(&initial_currency).unwrap_or(&0.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read initial currency
    let initial_currency = lines.next().unwrap().unwrap();
    
    // Read n1
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut pairs1 = vec![vec![String::new(); 2]; n1];
    let mut rates1 = vec![0.0; n1];
    
    // Read pairs1 and rates1
    for i in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        pairs1[i][0] = parts[0].to_string();
        pairs1[i][1] = parts[1].to_string();
        rates1[i] = parts[2].parse().unwrap();
    }
    
    // Read n2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut pairs2 = vec![vec![String::new(); 2]; n2];
    let mut rates2 = vec![0.0; n2];
    
    // Read pairs2 and rates2
    for i in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        pairs2[i][0] = parts[0].to_string();
        pairs2[i][1] = parts[1].to_string();
        rates2[i] = parts[2].parse().unwrap();
    }
    
    // Calculate and print result
    let result = Solution::max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);
}