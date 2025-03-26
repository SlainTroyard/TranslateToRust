use std::cmp;
use std::io::{self, BufRead};

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    // Create an index array with a fixed size of 100005 initialized to i32::MAX.
    let mut index = vec![i32::MAX; 100005];
    
    // Update the index array with the minimum index for each element.
    // Note: elements[i] is used as an index into the index vector.
    for (i, &elem) in elements.iter().enumerate() {
        // Update the value at index elem to be the minimum between the current value and the current index.
        if (elem as usize) < index.len() {
            index[elem as usize] = cmp::min(index[elem as usize], i as i32);
        }
    }
    
    let mut ans = Vec::with_capacity(groups.len());
    
    // Process each group.
    for &group in groups {
        // Initialize the answer for this group to i32::MAX.
        let mut best = i32::MAX;
        // Iterate j from 1 to sqrt(group) inclusive.
        let limit = (group as f64).sqrt() as i32;
        for j in 1..=limit {
            // Check if j is a divisor of group.
            if group % j == 0 {
                // The paired factor is group / j.
                let paired = group / j;
                // Only consider this factor if either j or paired has been recorded in the index.
                if index[j as usize] != i32::MAX || index[paired as usize] != i32::MAX {
                    best = cmp::min(best, cmp::min(index[j as usize], index[paired as usize]));
                }
            }
        }
        // If best remains i32::MAX then there was no valid assignment, so set it to -1.
        if best == i32::MAX {
            best = -1;
        }
        ans.push(best);
    }
    
    ans
}

fn main() {
    // Read all input from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    
    // Read first line, which should contain two integers: n (number of groups) and m (number of elements)
    // The input can be spread across multiple lines so we split accordingly.
    let mut first_numbers = Vec::new();
    while first_numbers.len() < 2 {
        if let Some(line) = lines.next() {
            for num in line.split_whitespace() {
                if let Ok(val) = num.parse::<i32>() {
                    first_numbers.push(val);
                }
            }
        } else {
            break;
        }
    }
    
    if first_numbers.len() < 2 {
        eprintln!("Error: Not enough input for n and m");
        return;
    }
    
    let n = first_numbers[0] as usize;
    let m = first_numbers[1] as usize;
    
    // Read next n numbers for groups.
    let mut groups = Vec::with_capacity(n);
    while groups.len() < n {
        if let Some(line) = lines.next() {
            for num in line.split_whitespace() {
                if let Ok(val) = num.parse::<i32>() {
                    groups.push(val);
                    if groups.len() == n {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
    if groups.len() < n {
        eprintln!("Error: Not enough input for groups");
        return;
    }
    
    // Read next m numbers for elements.
    let mut elements = Vec::with_capacity(m);
    while elements.len() < m {
        if let Some(line) = lines.next() {
            for num in line.split_whitespace() {
                if let Ok(val) = num.parse::<i32>() {
                    elements.push(val);
                    if elements.len() == m {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
    if elements.len() < m {
        eprintln!("Error: Not enough input for elements");
        return;
    }
    
    // Compute the answer using the assign_elements function.
    let ans = assign_elements(&groups, &elements);
    
    // Print the answer in the same format as the C code: each number followed by a space then a newline.
    // We use print! rather than println! to match the exact output format.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for num in ans {
        // Write each number followed by a space.
        // Using unwrap() here for simplicity as writing to stdout is unlikely to fail.
        write!(handle, "{} ", num).unwrap();
    }
    // Finish output with a newline.
    writeln!(handle).unwrap();
}