use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
        for _ in 0..pairs.len() {
            for i in 0..pairs.len() {
                let from_currency = &pairs[i][0];
                let to_currency = &pairs[i][1];
                let rate = rates[i];
                
                // Update best exchange rate for to_currency
                let from_value = *best.get(from_currency).unwrap_or(&0.0);
                let current_to_value = *best.get(to_currency).unwrap_or(&0.0);
                let new_to_value = from_value * rate;
                if new_to_value > current_to_value {
                    best.insert(to_currency.clone(), new_to_value);
                }
                
                // Update best exchange rate for from_currency
                let to_value = *best.get(to_currency).unwrap_or(&0.0);
                let current_from_value = *best.get(from_currency).unwrap_or(&0.0);
                let new_from_value = to_value / rate;
                if new_from_value > current_from_value {
                    best.insert(from_currency.clone(), new_from_value);
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
        let mut best: HashMap<String, f64> = HashMap::new();
        best.insert(initial_currency.clone(), 1.0);
        
        Self::bellman(&mut best, &pairs1, &rates1);
        Self::bellman(&mut best, &pairs2, &rates2);
        
        *best.get(&initial_currency).unwrap_or(&1.0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read initial currency
    let initial_currency = lines.next().unwrap()?.trim().to_string();
    
    // Read pairs1 and rates1
    let n1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs1 = vec![vec![String::new(), String::new()]; n1];
    let mut rates1 = vec![0.0; n1];
    
    for i in 0..n1 {
        let line = lines.next().unwrap()?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs1[i][0] = parts[0].to_string();
        pairs1[i][1] = parts[1].to_string();
        rates1[i] = parts[2].parse().unwrap();
    }
    
    // Read pairs2 and rates2
    let n2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut pairs2 = vec![vec![String::new(), String::new()]; n2];
    let mut rates2 = vec![0.0; n2];
    
    for i in 0..n2 {
        let line = lines.next().unwrap()?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs2[i][0] = parts[0].to_string();
        pairs2[i][1] = parts[1].to_string();
        rates2[i] = parts[2].parse().unwrap();
    }
    
    // Calculate and print result
    let result = Solution::max_amount(initial_currency, pairs1, rates1, pairs2, rates2);
    println!("{:.5}", result);
    
    Ok(())
}