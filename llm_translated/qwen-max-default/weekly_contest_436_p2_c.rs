use std::io;

fn assign_elements(groups: &Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = index[element as usize].min(i as i32);
    }

    let mut ans = vec![i32::MAX; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        for j in 1..=((group as f64).sqrt() as i32 + 1) {
            if group % j == 0 && (index[j as usize] != i32::MAX || index[group / j as usize] != i32::MAX) {
                ans[i] = ans[i].min(index[group / j as usize].min(index[j as usize]));
            }
        }
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    ans
}

fn main() {
    // Read the number of groups and elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");

    // Read the groups
    let mut groups = vec![0; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        groups[i] = input.trim().parse().expect("Failed to parse group");
    }

    // Read the elements
    let mut elements = vec![0; m];
    for i in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        elements[i] = input.trim().parse().expect("Failed to parse element");
    }

    // Assign elements to groups
    let ans = assign_elements(&groups, &elements);

    // Print the result
    for &val in &ans {
        print!("{} ", val);
    }
    println!();
}