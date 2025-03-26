use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn assign_elements(groups: &mut Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
        let mx = *elements.iter().max().unwrap();
        let mut target = vec![-1; (mx + 1) as usize];
        
        for (i, &x) in elements.iter().enumerate() {
            if x > mx || target[x as usize] >= 0 {
                continue;
            }
            let mut y = x;
            while y <= mx {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32;
                }
                y += x;
            }
        }

        for x in groups.iter_mut() {
            *x = target[*x as usize];
        }
        groups.clone()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap()?;
    let mut nm = first_line.split_whitespace();
    let n: usize = nm.next().unwrap().parse().unwrap();
    let m: usize = nm.next().unwrap().parse().unwrap();

    // Read groups
    let mut groups: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read elements
    let elements: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Process and print result
    let result = Solution::assign_elements(&mut groups, &elements);
    for &num in &result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}