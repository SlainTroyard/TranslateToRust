use std::io::{self, BufRead};

// Stack implementation for storing positions
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<i32> {
        self.data.last().copied()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Main solution function
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks = vec![Stack::new(len); 26]; // One stack for each letter
    let mut ans: i64 = 0;

    for (i, c) in s.chars().enumerate() {
        let c = (c as u8 - b'a') as usize;
        if !stacks[25 - c].is_empty() {
            // Found a match - calculate score and pop
            ans += (i as i64) - (stacks[25 - c].top().unwrap() as i64);
            stacks[25 - c].pop();
        } else {
            // No match - push current position
            stacks[c].push(i as i32);
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    if let Some(Ok(s)) = lines.next() {
        println!("{}", calculate_score(&s));
    }
}