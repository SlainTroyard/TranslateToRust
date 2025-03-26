use std::collections::HashMap;
use std::io::Read;

struct Solution;

impl Solution {
    /// Relax exchange rates using Bellman-Ford-like algorithm for each pair
    fn bellman(best: &mut HashMap<String, f64>, pairs: &[Vec<String>], rates: &[f64]) {
        // Run relaxation for as many iterations as there are pairs
        for _ in 0..pairs.len() {
            // Iterate through each currency pair and corresponding rate
            for (pair, &rate) in pairs.iter().zip(rates) {
                let from = &pair[0];
                let to = &pair[1];
                
                // Update the best rate for 'to' currency using 'from' currency's current rate
                if let Some(current_from) = best.get(from) {
                    let new_to = current_from * rate;
                    if new_to > *best.get(to).unwrap_or(&0.0) {
                        best.insert(to.clone(), new_to);
                    }
                }
                
                // Update the best rate for 'from' currency using 'to' currency's current rate
                if let Some(current_to) = best.get(to) {
                    let new_from = current_to / rate;
                    if new_from > *best.get(from).unwrap_or(&0.0) {
                        best.insert(from.clone(), new_from);
                    }
                }
            }
        }
    }

    /// Calculate maximum possible amount of initial currency after all exchanges
    fn max_amount(
        initial_currency: String,
        pairs1: &[Vec<String>],
        rates1: &[f64],
        pairs2: &[Vec<String>],
        rates2: &[f64],
    ) -> f64 {
        let mut best = HashMap::new();
        // Start with 1 unit of the initial currency
        best.insert(initial_currency.clone(), 1.0);
        // Apply exchange rate optimizations for both sets of pairs
        Self::bellman(&mut best, pairs1, rates1);
        Self::bellman(&mut best, pairs2, rates2);
        // Return the optimized amount for the initial currency
        *best.get(&initial_currency).unwrap_or(&0.0)
    }
}

fn main() {
    // Read all input at once and split into tokens
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Parse initial currency
    let initial_currency = tokens.next().unwrap().to_string();
    
    // Parse first set of pairs and rates
    let n1: usize = tokens.next().unwrap().parse().unwrap();
    let mut pairs1 = Vec::with_capacity(n1);
    let mut rates1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let from = tokens.next().unwrap().to_string();
        let to = tokens.next().unwrap().to_string();
        let rate: f64 = tokens.next().unwrap().parse().unwrap();
        pairs1.push(vec![from, to]);
        rates1.push(rate);
    }
    
    // Parse second set of pairs and rates
    let n2: usize = tokens.next().unwrap().parse().unwrap();
    let mut pairs2 = Vec::with_capacity(n2);
    let mut rates2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let from = tokens.next().unwrap().to_string();
        let to = tokens.next().unwrap().to_string();
        let rate: f64 = tokens.next().unwrap().parse().unwrap();
        pairs2.push(vec![from, to]);
        rates2.push(rate);
    }
    
    // Calculate and print result with exactly 5 decimal places
    let result = Solution::max_amount(initial_currency, &pairs1, &rates1, &pairs2, &rates2);
    println!("{:.5}", result);
}