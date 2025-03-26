use std::io::{self, BufRead};
use std::cmp;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    // Initialize index array with i32::MAX
    let mut index = vec![i32::MAX; 100005];
    
    // Update index array with minimum indices of elements
    for (i, &element) in elements.iter().enumerate() {
        let element = element as usize;
        if element < index.len() {
            index[element] = cmp::min(index[element], i as i32);
        }
    }
    
    // Process each group
    let mut ans = vec![i32::MAX; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        for j in 1..=((group as f64).sqrt() as i32) {
            if group % j == 0 {
                let factor1 = j as usize;
                let factor2 = (group / j) as usize;
                if factor1 < index.len() && index[factor1] != i32::MAX {
                    ans[i] = cmp::min(ans[i], index[factor1]);
                }
                if factor2 < index.len() && factor2 != factor1 && index[factor2] != i32::MAX {
                    ans[i] = cmp::min(ans[i], index[factor2]);
                }
            }
        }
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let first_line = lines.next().unwrap()?;
    let mut numbers = first_line.split_whitespace();
    let n: usize = numbers.next().unwrap().parse().unwrap();
    let m: usize = numbers.next().unwrap().parse().unwrap();

    let mut groups = Vec::with_capacity(n);
    let mut elements = Vec::with_capacity(m);

    for _ in 0..n {
        let line = lines.next().unwrap()?;
        groups.push(line.trim().parse().unwrap());
    }

    for _ in 0..m {
        let line = lines.next().unwrap()?;
        elements.push(line.trim().parse().unwrap());
    }

    // Process input
    let result = assign_elements(&groups, &elements);

    // Output result
    for &num in &result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}