use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::io::{self, BufRead, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // To match the same I/O interface as C++ code, we use stdin and stdout.
    // Create a buffered reader for standard input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read the first line containing n, m, k.
    reader.read_line(&mut input)?;
    let mut first_line = input.split_whitespace();
    let n: usize = first_line
        .next()
        .ok_or("Missing n")?
        .parse()?;
    let m: usize = first_line
        .next()
        .ok_or("Missing m")?
        .parse()?;
    let mut k: i32 = first_line
        .next()
        .ok_or("Missing k")?
        .parse()?;
    
    // Read the grid values.
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        input.clear();
        // Read enough tokens for the row.
        while input.split_whitespace().count() < m {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            input.push_str(&line);
        }
        let row_values: Vec<i32> = input
            .split_whitespace()
            .take(m)
            .map(|token| token.parse::<i32>())
            .collect::<Result<_, _>>()?;
        grid[i] = row_values;
    }
    
    // Read the limits vector (n values).
    let mut limits_line = String::new();
    // If the limits are not on a new line, we might need to read multiple lines.
    let mut limits_tokens = Vec::new();
    while limits_tokens.len() < n {
        limits_line.clear();
        reader.read_line(&mut limits_line)?;
        limits_tokens.extend(limits_line.split_whitespace().map(String::from));
    }
    let limits: Vec<usize> = limits_tokens
        .into_iter()
        .take(n)
        .map(|token| token.parse::<usize>())
        .collect::<Result<_, _>>()?;
    
    // Sort each row in descending order.
    for row in grid.iter_mut() {
        // sort descending using sort_by with reverse order.
        row.sort_by(|a, b| b.cmp(a));
    }
    
    // Create a max heap (BinaryHeap) to simulate the priority queue.
    // Each element in the heap will be a tuple: (value, row index, column index)
    // BinaryHeap in Rust is a max-heap by default based on Ord.
    let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
    for (i, row) in grid.iter().enumerate() {
        // Push the top value from each row.
        // It is guaranteed that each row has at least one element because m >= 1.
        heap.push((row[0], i, 0));
    }
    
    // Process the heap up to k times.
    let mut ans: i64 = 0;
    while k > 0 && !heap.is_empty() {
        // Extract the current max element.
        let (val, r, c) = heap.pop().unwrap();
        
        // If the current column index is outside the limit for this row, skip.
        if c >= limits[r] {
            continue;
        }
        
        // Add the value to the accumulated answer.
        ans += val as i64;
        k -= 1;
        
        // If there is a next column available in the same row, push that element into the heap.
        if c + 1 < m {
            heap.push((grid[r][c + 1], r, c + 1));
        }
    }
    
    // Print the result, matching the exact output format.
    writeln!(io::stdout(), "{}", ans)?;
    
    Ok(())
}