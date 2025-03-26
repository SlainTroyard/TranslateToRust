use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn assign_elements(mut groups: Vec<i32>, elements: &[i32]) -> Vec<i32> {
        // Find maximum element
        let mx = match elements.iter().max() {
            Some(&max) => max,
            None => return groups,
        };
        
        // Initialize target array with -1
        let mut target = vec![-1; (mx + 1) as usize];
        
        // Process elements
        for (i, &x) in elements.iter().enumerate() {
            if x > mx || target[x as usize] >= 0 {
                continue;
            }
            
            // Mark all multiples of x
            let mut y = x;
            while y <= mx {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32;
                }
                y += x;
            }
        }
        
        // Update groups based on target array
        for x in groups.iter_mut() {
            *x = target[*x as usize];
        }
        
        groups
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Parse groups
    let second_line = lines.next().unwrap().unwrap();
    let mut groups = Vec::with_capacity(n);
    let mut iter = second_line.split_whitespace();
    for _ in 0..n {
        if let Some(val) = iter.next() {
            groups.push(val.parse::<i32>().unwrap());
        }
    }
    
    // Parse elements
    let third_line = lines.next().unwrap().unwrap();
    let mut elements = Vec::with_capacity(m);
    let mut iter = third_line.split_whitespace();
    for _ in 0..m {
        if let Some(val) = iter.next() {
            elements.push(val.parse::<i32>().unwrap());
        }
    }
    
    // Solve and get result
    let result = Solution::assign_elements(groups, &elements);
    
    // Output result
    for (i, val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    println!();
}