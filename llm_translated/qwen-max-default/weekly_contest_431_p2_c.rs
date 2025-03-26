use std::io::{self, BufRead, Write};

// Stack implementation for storing positions
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack { data: Vec::with_capacity(capacity) }
    }

    fn push(&mut self, value: i32) {
        if self.data.len() < self.data.capacity() {
            self.data.push(value);
        }
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
        let idx = (c as u8 - b'a') as usize;
        if !stacks[25 - idx].is_empty() {
            // Found a match - calculate score and pop
            if let Some(top) = stacks[25 - idx].pop() {
                ans += (i as i64) - (top as i64);
            }
        } else {
            // No match - push current position
            stacks[idx].push(i as i32);
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the input string
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let input = input.trim(); // Remove any trailing newline

    // Calculate the score and print the result
    let result = calculate_score(input);
    writeln!(stdout, "{}", result).unwrap();
}