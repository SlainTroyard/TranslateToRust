use std::io::{self, BufRead};
use std::cmp;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    // Create an index array to store the minimum position of each element
    let mut index = vec![i32::MAX; 100005];
    
    // Record the minimum position for each element
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = cmp::min(index[element as usize], i as i32);
    }
    
    let mut ans = vec![i32::MAX; groups.len()];
    
    // Process each group
    for (i, &group) in groups.iter().enumerate() {
        // Check all divisors of the group value
        let sqrt_limit = (group as f64).sqrt() as i32;
        for j in 1..=sqrt_limit {
            if group % j == 0 {
                let divisor1 = j;
                let divisor2 = group / j;
                
                // Check if either divisor exists in the elements array
                if index[divisor1 as usize] != i32::MAX || index[divisor2 as usize] != i32::MAX {
                    // Take the minimum position
                    let min_pos = cmp::min(
                        index[divisor1 as usize],
                        index[divisor2 as usize]
                    );
                    ans[i] = cmp::min(ans[i], min_pos);
                }
            }
        }
        
        // If no valid element found, set to -1
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Read groups
    let groups_line = lines.next().unwrap().unwrap();
    let groups: Vec<i32> = groups_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read elements
    let elements_line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = elements_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Process and print results
    let result = assign_elements(&groups, &elements);
    
    // Print the result with spaces between elements
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}