use std::io::{self, BufRead};

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    // Create an index array to store the first occurrence of each element
    let mut index = vec![i32::MAX; 100005];
    
    // Record the first occurrence of each element
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = index[element as usize].min(i as i32);
    }
    
    // Calculate the result for each group
    let mut ans = vec![i32::MAX; groups.len()];
    
    for (i, &group) in groups.iter().enumerate() {
        // Check all divisors of the group value
        let sqrt_limit = (group as f64).sqrt() as i32;
        for j in 1..=sqrt_limit {
            if group % j == 0 {
                let div1 = j;
                let div2 = group / j;
                
                // If either divisor exists in the elements array
                if index[div1 as usize] != i32::MAX || index[div2 as usize] != i32::MAX {
                    // Take the minimum occurrence index
                    let min_index = index[div1 as usize].min(index[div2 as usize]);
                    ans[i] = ans[i].min(min_index);
                }
            }
        }
        
        // If no valid divisor was found, set to -1
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|l| l.unwrap());
    
    // Parse the first line containing n and m
    let line = iterator.next().unwrap();
    let mut parts = line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    
    // Parse the groups array
    let line = iterator.next().unwrap();
    let groups: Vec<i32> = line.split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Parse the elements array
    let line = iterator.next().unwrap();
    let elements: Vec<i32> = line.split_whitespace()
        .take(m)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Compute the result
    let result = assign_elements(&groups, &elements);
    
    // Print the result
    for (i, &val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    println!();
}