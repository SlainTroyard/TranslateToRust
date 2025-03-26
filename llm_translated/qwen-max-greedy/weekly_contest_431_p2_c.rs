use std::io::{self, Write};

// Stack implementation for storing positions
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack { data: Vec::with_capacity(capacity) }
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
    let mut stacks: [Stack; 26] = core::array::from_fn(|_| Stack::new(len));
    let mut ans: i64 = 0;

    for (i, c) in s.chars().enumerate() {
        let index = (c as u8 - b'a') as usize;
        if !stacks[25 - index].is_empty() {
            // Found a match - calculate score and pop
            if let Some(top) = stacks[25 - index].pop() {
                ans += (i as i64) - (top as i64);
            }
        } else {
            // No match - push current position
            stacks[index].push(i as i32);
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Remove any trailing newline

    let result = calculate_score(input);

    println!("{}", result);
}