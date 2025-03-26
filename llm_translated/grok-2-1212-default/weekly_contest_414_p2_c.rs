use std::io::{self, BufRead};

fn max_possible_score(start: &mut [i32], start_size: usize, d: i32) -> i32 {
    // Sort array
    for i in 0..start_size - 1 {
        for j in 0..start_size - i - 1 {
            if start[j] > start[j + 1] {
                start.swap(j, j + 1);
            }
        }
    }
    
    // Binary search
    let mut left = 0;
    let mut right = (start[start_size - 1] + d - start[0]) / (start_size as i32 - 1) + 1;
    
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        
        // Check function
        let mut sum = i64::MIN;
        let mut valid = true;
        for i in 0..start_size {
            sum = sum.max(sum + mid as i64);
            sum = sum.max(start[i] as i64);
            if sum > start[i] as i64 + d as i64 {
                valid = false;
                break;
            }
        }
        
        if valid {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read start_size and d
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let start_size: usize = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();

    // Read start array
    let mut start = Vec::with_capacity(start_size);
    for _ in 0..start_size {
        let line = lines.next().unwrap()?;
        start.push(line.trim().parse().unwrap());
    }

    // Calculate and print result
    let result = max_possible_score(&mut start, start_size, d);
    println!("{}", result);

    Ok(())
}