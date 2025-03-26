use std::io::{self, Read};

// Stack implementation for storing positions
struct Stack {
    data: Vec<usize>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: usize) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<usize> {
        self.data.pop()
    }

    fn top(&self) -> Option<usize> {
        self.data.last().copied()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Main solution function
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: [Stack; 26] = std::array::from_fn(|_| Stack::new());
    let mut ans: i64 = 0;

    for (i, c) in s.chars().enumerate() {
        let c_index = (c as u8 - b'a') as usize;
        if !stacks[25 - c_index].is_empty() {
            // Found a match - calculate score and pop
            if let Some(top) = stacks[25 - c_index].top() {
                ans += i as i64 - top as i64;
            }
            stacks[25 - c_index].pop();
        } else {
            // No match - push current position
            stacks[c_index].push(i);
        }
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Process each line of input
    for line in input.lines() {
        let score = calculate_score(line);
        println!("{}", score);
    }

    Ok(())
}