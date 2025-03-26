use std::collections::HashMap;
use std::io::{self, BufRead};

fn bellman(best: &mut HashMap<String, f64>, pairs: &Vec<Vec<String>>, rates: &Vec<f64>) {
    for _ in 0..pairs.len() {
        for i in 0..pairs.len() {
            let pair = &pairs[i];
            let rate = rates[i];
            best.entry(pair[1].clone()).and_modify(|e| *e = (*e).max(best[&pair[0]] * rate)).or_insert(best[&pair[0]] * rate);
            best.entry(pair[0].clone()).and_modify(|e| *e = (*e).max(best[&pair[1]] / rate)).or_insert(best[&pair[1]] / rate);
        }
    }
}

struct Solution;

impl Solution {
    fn max_amount(initial_currency: &str, pairs1: &Vec<Vec<String>>, rates1: &Vec<f64>, pairs2: &Vec<Vec<String>>, rates2: &Vec<f64>) -> f64 {
        let mut best: HashMap<String, f64> = HashMap::new();
        best.insert(initial_currency.to_string(), 1.0);
        bellman(&mut best, pairs1, rates1);
        bellman(&mut best, pairs2, rates2);
        *best.get(initial_currency).unwrap_or(&0.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let initial_currency = lines.next().unwrap().unwrap();
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut pairs1: Vec<Vec<String>> = vec![vec!["".to_string(); 2]; n1];
    let mut rates1: Vec<f64> = vec![0.0; n1];

    for i in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        pairs1[i][0] = parts[0].to_string();
        pairs1[i][1] = parts[1].to_string();
        rates1[i] = parts[2].parse().unwrap();
    }

    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut pairs2: Vec<Vec<String>> = vec![vec!["".to_string(); 2]; n2];
    let mut rates2: Vec<f64> = vec![0.0; n2];

    for i in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        pairs2[i][0] = parts[0].to_string();
        pairs2[i][1] = parts[1].to_string();
        rates2[i] = parts[2].parse().unwrap();
    }

    let result = Solution::max_amount(&initial_currency, &pairs1, &rates1, &pairs2, &rates2);
    println!("{:.5}", result);
}