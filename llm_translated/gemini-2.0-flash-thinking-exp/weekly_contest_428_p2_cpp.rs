use std::collections::HashMap;
use std::io;
use std::io::BufRead;

struct Solution {}

impl Solution {
    fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
        for _relax_iteration in 0..pairs.len() {
            for i in 0..pairs.len() {
                let pair = &pairs[i];
                let rate = rates[i];
                let currency1 = &pair[0];
                let currency2 = &pair[1];

                if let (Some(&val1), Some(_)) = (best.get(currency1), best.get(currency2)) {
                    let new_val2 = val1 * rate;
                    best.entry(currency2.clone())
                        .and_modify(|val| *val = val.max(new_val2))
                        .or_insert(new_val2);
                }
                if let (Some(_), Some(&val2)) = (best.get(currency1), best.get(currency2)) {
                    let new_val1 = val2 / rate;
                    best.entry(currency1.clone())
                        .and_modify(|val| *val = val.max(new_val1))
                        .or_insert(new_val1);
                }
            }
        }
    }

    fn maxAmount(
        &self,
        initial_currency: String,
        pairs1: Vec<Vec<String>>,
        rates1: Vec<f64>,
        pairs2: Vec<Vec<String>>,
        rates2: Vec<f64>,
    ) -> f64 {
        let mut best: HashMap<String, f64> = HashMap::new();
        best.insert(initial_currency.clone(), 1.0);
        Solution::bellman(&mut best, &pairs1, &rates1);
        Solution::bellman(&mut best, &pairs2, &rates2);
        *best.get(&initial_currency).unwrap_or(&0.0) // Return 0.0 if initial currency is not found after bellman, though it should always be present
    }
}

fn main() {
    let solution = Solution {};
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read initialCurrency
    let initial_currency = lines.next().unwrap().unwrap();

    // Read n1
    let n1_str = lines.next().unwrap().unwrap();
    let n1: usize = n1_str.trim().parse().unwrap();

    // Read pairs1 and rates1
    let mut pairs1: Vec<Vec<String>> = Vec::new();
    let mut rates1: Vec<f64> = Vec::new();
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs1.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates1.push(parts[2].trim().parse().unwrap());
    }

    // Read n2
    let n2_str = lines.next().unwrap().unwrap();
    let n2: usize = n2_str.trim().parse().unwrap();

    // Read pairs2 and rates2
    let mut pairs2: Vec<Vec<String>> = Vec::new();
    let mut rates2: Vec<f64> = Vec::new();
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        pairs2.push(vec![parts[0].to_string(), parts[1].to_string()]);
        rates2.push(parts[2].trim().parse().unwrap());
    }

    let result = solution.maxAmount(
        initial_currency,
        pairs1,
        rates1,
        pairs2,
        rates2,
    );
    println!("{:.5}", result);
}