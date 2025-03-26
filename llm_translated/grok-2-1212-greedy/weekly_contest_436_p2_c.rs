use std::io::{self, BufRead};
use std::cmp;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    for (i, &element) in elements.iter().enumerate() {
        let element = element as usize;
        index[element] = cmp::min(index[element], i as i32);
    }

    let mut ans = vec![i32::MAX; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        ans[i] = i32::MAX;
        for j in 1..=(group as f64).sqrt() as i32 {
            if group % j == 0 {
                let j_index = index[j as usize];
                let group_div_j_index = index[(group / j) as usize];
                if j_index != i32::MAX || group_div_j_index != i32::MAX {
                    ans[i] = cmp::min(ans[i], cmp::min(j_index, group_div_j_index));
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

    // Read n and m
    let first_line = lines.next().unwrap()?;
    let mut nm = first_line.split_whitespace();
    let n: usize = nm.next().unwrap().parse().unwrap();
    let m: usize = nm.next().unwrap().parse().unwrap();

    // Read groups
    let groups: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read elements
    let elements: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Process and print result
    let result = assign_elements(&groups, &elements);
    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}