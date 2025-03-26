use std::io;

struct Solution {}

impl Solution {
    fn button_with_longest_time(events: &Vec<Vec<i32>>) -> i32 {
        let mut idx = events[0][0];
        let mut max_diff = events[0][1];
        
        for i in 1..events.len() {
            let p = &events[i - 1];
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
    
    // Read the number of events
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // Initialize events vector
    let mut events: Vec<Vec<i32>> = vec![vec![0; 2]; n];
    
    // Read each event (index and time)
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        
        events[i][0] = values[0]; // button index
        events[i][1] = values[1]; // time of press
    }
    
    let solution = Solution {};
    let result = solution.button_with_longest_time(&events);
    println!("{}", result);
}