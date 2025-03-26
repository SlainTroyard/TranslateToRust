use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn assign_elements(groups: &mut Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
        // Find the maximum value in elements
        let mx = *elements.iter().max().unwrap_or(&0);
        
        // Create a target array with default value -1
        let mut target = vec![-1; mx as usize + 1];
        
        // Populate the target array
        for i in 0..elements.len() {
            let x = elements[i];
            if x > mx || target[x as usize] >= 0 {
                continue;
            }
            
            // For each multiple of x up to mx, assign index i if not already assigned
            let mut y = x;
            while y <= mx {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32;
                }
                y += x;
            }
        }

        // Replace each value in groups with its corresponding target value
        for x in groups.iter_mut() {
            *x = target[*x as usize];
        }
        
        groups.clone()
    }
}

fn main() {
    // Set up input reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Read groups array
    let line = lines.next().unwrap().unwrap();
    let mut groups: Vec<i32> = line.split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read elements array
    let line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = line.split_whitespace()
        .take(m)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Process and output result
    let result = Solution::assign_elements(&mut groups, &elements);
    
    // Print result
    for i in 0..n {
        print!("{} ", result[i]);
    }
    println!();
}