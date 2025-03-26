use std::io;

struct Solution;

impl Solution {
    fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut idx = events[0][0];
        let mut max_diff = events[0][1];
        
        for i in 1..events.len() {
            let p = &events[i-1];
            let q = &events[i];
            let d = q[1] - p[1];
            if d > max_diff || (d == max_diff && q[0] < idx) {
                idx = q[0];
                max_diff = d;
            }
        }
        
        idx
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = tokens[0] as usize;
    assert!(tokens.len() == 2 * n + 1, "Incorrect number of tokens");
    
    let mut events = Vec::with_capacity(n);
    for i in 0..n {
        let idx = tokens[1 + 2 * i];
        let time = tokens[2 + 2 * i];
        events.push(vec![idx, time]);
    }
    
    let solution = Solution;
    let result = solution.button_with_longest_time(events);
    println!("{}", result);
}