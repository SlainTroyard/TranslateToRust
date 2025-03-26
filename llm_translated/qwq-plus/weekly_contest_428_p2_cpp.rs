use std::collections::HashMap;

struct Solution;

impl Solution {
    fn bellman(
        best: &mut HashMap<String, f64>,
        pairs: &Vec<Vec<String>>,
        rates: &Vec<f64>,
    ) {
        for _ in 0..pairs.len() {
            for i in 0..pairs.len() {
                let from = &pairs[i][0];
                let to = &pairs[i][1];
                let rate = rates[i];

                // Update 'to' value
                let current_from_val = *best.get(from).unwrap_or(&0.0);
                let candidate_to = current_from_val * rate;
                let current_to_val = *best.get(to).unwrap_or(&0.0);
                let new_to_val = candidate_to.max(current_to_val);
                best.insert(to.clone(), new_to_val);

                // Update 'from' value via 'to'
                let current_to_val = *best.get(to).unwrap_or(&0.0);
                let candidate_from = current_to_val / rate;
                let current_from_val = *best.get(from).unwrap_or(&0.0);
                let new_from_val = candidate_from.max(current_from_val);
                best.insert(from.clone(), new_from_val);
            }
        }
    }

    pub fn max_amount(
        &self,
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
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx = 0;

    let initial_currency = tokens[idx].to_string();
    idx += 1;

    let n1: usize = tokens[idx].parse().expect("Invalid n1");
    idx += 1;

    let mut pairs1 = Vec::new();
    let mut rates1 = Vec::new();
    for _ in 0..n1 {
        let a = tokens[idx].to_string();
        idx += 1;
        let b = tokens[idx].to_string();
        idx += 1;
        let rate: f64 = tokens[idx].parse().expect("Invalid rate");
        idx += 1;
        pairs1.push(vec![a, b]);
        rates1.push(rate);
    }

    let n2: usize = tokens[idx].parse().expect("Invalid n2");
    idx += 1;

    let mut pairs2 = Vec::new();
    let mut rates2 = Vec::new();
    for _ in 0..n2 {
        let a = tokens[idx].to_string();
        idx += 1;
        let b = tokens[idx].to_string();
        idx += 1;
        let rate: f64 = tokens[idx].parse().expect("Invalid rate");
        idx += 1;
        pairs2.push(vec![a, b]);
        rates2.push(rate);
    }

    let solution = Solution;
    let result = solution.max_amount(
        initial_currency,
        pairs1,
        rates1,
        pairs2,
        rates2,
    );

    println!("{result:.5}");
}