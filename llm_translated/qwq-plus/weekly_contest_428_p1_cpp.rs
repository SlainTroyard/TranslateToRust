struct Solution;

impl Solution {
    pub fn button_with_longest_time(events: Vec<(i32, i32)>) -> i32 {
        let mut idx = events[0].0;
        let mut max_diff = events[0].1;
        for i in 1..events.len() {
            let prev = &events[i - 1];
            let curr = &events[i];
            let d = curr.1 - prev.1;
            if d > max_diff || (d == max_diff && curr.0 < idx) {
                idx = curr.0;
                max_diff = d;
            }
        }
        idx
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().expect("Invalid n");
    let mut events = Vec::with_capacity(n);

    for _ in 0..n {
        let idx = tokens.next().unwrap().parse().expect("Invalid index");
        let time = tokens.next().unwrap().parse().expect("Invalid time");
        events.push((idx, time));
    }

    let result = Solution::button_with_longest_time(events);
    println!("{}", result);
}