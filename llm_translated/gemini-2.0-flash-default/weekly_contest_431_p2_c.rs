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

    fn top(&self) -> Option<usize> {
        self.data.last().copied()
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
        if let Some(top) = stacks[25 - c_index].top() {
            // Found a match - calculate score and pop
            ans += (i as i64 - top as i64) as i64;
            stacks[25 - c_index].pop();
        } else {
            // No match - push current position
            stacks[c_index].push(i);
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim();

    let result = calculate_score(s);
    println!("{}", result);

    Ok(())
}