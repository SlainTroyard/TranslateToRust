// Problem: Weekly Contest 431 Problem 2
use std::io::{self, Read};
use std::collections::VecDeque;

// Helper struct to implement a simple stack
#[derive(Debug)]
struct Stack {
    data: VecDeque<usize>, // VecDeque used to implement stack behavior efficiently
}

impl Stack {
    // Creates a new empty stack
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    // Pushes a value onto the stack
    fn push(&mut self, value: usize) {
        self.data.push_back(value);
    }

    // Pops a value from the stack (returns None if empty)
    fn pop(&mut self) -> Option<usize> {
        self.data.pop_back()
    }

    // Gets the top value of the stack without removing it (returns None if empty)
    fn top(&self) -> Option<usize> {
        self.data.back().cloned()
    }

    // Checks if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Main solution function
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: Vec<Stack> = (0..26).map(|_| Stack::new()).collect(); // One stack for each letter
    let mut ans: i64 = 0;

    for (i, ch) in s.chars().enumerate() {
        let c = (ch as u8 - b'a') as usize;
        if !stacks[25 - c].is_empty() {
            // Found a match - calculate score and pop
            let top_idx = stacks[25 - c].top().unwrap(); // Safe unwrap because is_empty() checked
            ans += (i - top_idx) as i64;
            stacks[25 - c].pop();
        } else {
            // No match - push current position
            stacks[c].push(i);
        }
    }

    ans
}

fn main() {
    // Read input string (similar to using `scanf` in C)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    // Since the input string might contain newline characters, trim them
    let input = input.trim();

    // Calculate the result and print it
    let result = calculate_score(input);
    println!("{}", result); // Matches the format of the original C program
}