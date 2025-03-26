use std::io;
use std::io::Read;

// Stack implementation using Vec
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

    fn top(&self) -> Option<&usize> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: Vec<Stack> = vec![Stack::new(); 26]; // One stack for each letter
    let mut ans: i64 = 0;

    for (i, c) in s.chars().enumerate() {
        let c_index = (c as u8 - b'a') as usize;
        let stack_index = 25 - c_index;

        if !stacks[stack_index].is_empty() {
            // Found a match - calculate score and pop
            if let Some(&top) = stacks[stack_index].top() {
                ans += (i as i64) - (top as i64);
            }
            stacks[stack_index].pop();
        } else {
            // No match - push current position
            stacks[c_index].push(i);
        }
    }

    ans
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let score = calculate_score(s);
    println!("{}", score);
}