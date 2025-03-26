use std::io::{self, BufRead};
use std::cmp::min;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    
    // Store the minimum index for each element
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = min(index[element as usize], i as i32);
    }
    
    let mut ans = vec![i32::MAX; groups.len()];
    
    // For each group, find the minimum index of its divisors
    for (i, &group) in groups.iter().enumerate() {
        let sqrt_val = (group as f64).sqrt() as i32;
        for j in 1..=sqrt_val {
            if group % j == 0 {
                // Check if j or group/j is in elements
                if index[j as usize] != i32::MAX || index[(group / j) as usize] != i32::MAX {
                    let div1_index = index[j as usize];
                    let div2_index = index[(group / j) as usize];
                    ans[i] = min(ans[i], min(div1_index, div2_index));
                }
            }
        }
        // If no divisor found, set to -1
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m from the first line
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Read groups array
    let groups_line = lines.next().unwrap()?;
    let groups: Vec<i32> = groups_line.split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read elements array
    let elements_line = lines.next().unwrap()?;
    let elements: Vec<i32> = elements_line.split_whitespace()
        .take(m)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the assignment function
    let ans = assign_elements(&groups, &elements);
    
    // Print the results with spaces between them
    for (i, &val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}