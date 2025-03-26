use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    fn bellman(
        best: &mut HashMap<String, f64>,
        pairs: &Vec<Vec<String>>,
        rates: &Vec<f64>,
    ) {
        for _relax_iteration in 0..pairs.len() {
            for i in 0..pairs.len() {
                let currency1 = &pairs[i][1];
                let currency0 = &pairs[i][0];
                let rate = rates[i];

                // Relax the edge from currency0 to currency1
                if let Some(val0) = best.get(currency0).copied() {
                    let new_val = val0 * rate;
                    best.entry(currency1.clone())
                        .and_modify(|e| *e = e.max(new_val))
                        .or_insert(new_val);
                }

                // Relax the edge from currency1 to currency0
                if let Some(val1) = best.get(currency1).copied() {
                    let new_val = val1 / rate;
                    best.entry(currency0.clone())
                        .and_modify(|e| *e = e.max(new_val))
                        .or_insert(new_val);
                }
            }
        }
    }

    fn max_amount(
        &self,
        initial_currency: String,
        pairs1: &Vec<Vec<String>>,
        rates1: &Vec<f64>,
        pairs2: &Vec<Vec<String>>,
        rates2: &Vec<f64>,
    ) -> f64 {
        let mut best: HashMap<String, f64> = HashMap::new();
        best.insert(initial_currency.clone(), 1.0);

        Solution::bellman(&mut best, pairs1, rates1);
        Solution::bellman(&mut best, pairs2, rates2);

        *best.get(&initial_currency).unwrap_or(&0.0)
    }
}

fn main() -> io::Result<()> {
    let mut initial_currency = String::new();
    io::stdin().read_line(&mut initial_currency).unwrap();
    let initial_currency = initial_currency.trim().to_string();

    let mut n1_str = String::new();
    io::stdin().read_line(&mut n1_str).unwrap();
    let n1: usize = n1_str.trim().parse().unwrap();

    let mut pairs1: Vec<Vec<String>> = Vec::with_capacity(n1);
    let mut rates1: Vec<f64> = Vec::with_capacity(n1);

    for _ in 0..n1 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs1.push(vec![
            parts[0].to_string(),
            parts[1].to_string(),
        ]);
        rates1.push(parts[2].parse().unwrap());
    }

    let mut n2_str = String::new();
    io::stdin().read_line(&mut n2_str).unwrap();
    let n2: usize = n2_str.trim().parse().unwrap();

    let mut pairs2: Vec<Vec<String>> = Vec::with_capacity(n2);
    let mut rates2: Vec<f64> = Vec::with_capacity(n2);

    for _ in 0..n2 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs2.push(vec![
            parts[0].to_string(),
            parts[1].to_string(),
        ]);
        rates2.push(parts[2].parse().unwrap());
    }

    let solution = Solution {};
    let result = solution.max_amount(
        initial_currency,
        &pairs1,
        &rates1,
        &pairs2,
        &rates2,
    );

    println!("{:.5}", result);

    Ok(())
}