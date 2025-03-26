use std::io::{self, BufRead, Write};

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = index[element as usize].min(i as i32);
    }

    let mut ans = vec![i32::MAX; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        for j in 1..=((group as f64).sqrt() as i32) {
            if group % j == 0 && (index[j as usize] != i32::MAX || index[(group / j) as usize] != i32::MAX) {
                ans[i] = ans[i].min(index[j as usize].min(index[(group / j) as usize]));
            }
        }
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the first line to get n and m
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let m: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Read the groups
    let mut groups = vec![0; n];
    for i in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        groups[i] = input.trim().parse().expect("Invalid input");
    }

    // Read the elements
    let mut elements = vec![0; m];
    for i in 0..m {
        input.clear();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        elements[i] = input.trim().parse().expect("Invalid input");
    }

    // Assign elements to groups
    let ans = assign_elements(&groups, &elements);

    // Print the result
    for (i, &result) in ans.iter().enumerate() {
        write!(&mut stdout, "{}", result).expect("Failed to write output");
        if i < ans.len() - 1 {
            write!(&mut stdout, " ").expect("Failed to write output");
        }
    }
    writeln!(&mut stdout).expect("Failed to write output");
}