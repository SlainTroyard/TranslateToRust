// Problem: Weekly Contest 431 Problem 2
use std::io::{self, Read};
use std::collections::VecDeque;

// Stack implementation for storing positions
struct Stack {
    data: VecDeque<usize>, // Using VecDeque for efficient push and pop
}

impl Stack {
    // Initialize a new stack
    fn new() -> Self {
        Stack {
            data: VecDeque::new(),
        }
    }

    // Push a value onto the stack
    fn push(&mut self, value: usize) {
        self.data.push_back(value);
    }

    // Pop a value from the stack
    fn pop(&mut self) -> Option<usize> {
        self.data.pop_back()
    }

    // Get the top value of the stack
    fn top(&self) -> Option<usize> {
        self.data.back().copied()
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Main solution function
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: Vec<Stack> = (0..26).map(|_| Stack::new()).collect(); // One stack for each letter
    let mut ans: i64 = 0;

    for (i, c) in s.chars().enumerate() {
        let c_index = (c as u8 - b'a') as usize;
        let complement_index = 25 - c_index;

        if !stacks[complement_index].is_empty() {
            // Found a match - calculate score and pop
            if let Some(top) = stacks[complement_index].top() {
                ans += (i - top) as i64;
                stacks[complement_index].pop();
            }
        } else {
            // No match - push current position
            stacks[c_index].push(i);
        }
    }

    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // The input is a single string (no newlines in the input string)
    let s = input.trim();

    // Calculate the score and print the result
    let result = calculate_score(s);
    println!("{}", result);
}