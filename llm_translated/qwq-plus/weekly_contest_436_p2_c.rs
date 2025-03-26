use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = tokens[0] as usize;
    let m = tokens[1] as usize;
    let groups = &tokens[2..2 + n];
    let elements = &tokens[2 + n..2 + n + m];
    let ans = assign_elements(groups, elements);
    for &x in &ans {
        print!("{} ", x);
    }
    println!();
}

/// Translated from the C function to find the minimal indices for each group.
///
/// # Arguments
///
/// * `groups` - Slice of integers representing the groups to process.
/// * `elements` - Slice of integers representing the elements array.
///
/// # Returns
///
/// A vector of integers where each element is the minimal index found for the corresponding group, or -1 if none found.
fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005]; // Elements can be up to 1e5, so 100005 covers all possible values.
    for (i, &e) in elements.iter().enumerate() {
        let e_usize = e as usize;
        if e_usize >= 100005 {
            panic!("Element value out of bounds");
        }
        // Track the earliest occurrence of each element's value.
        index[e_usize] = index[e_usize].min(i as i32);
    }

    let mut ans = Vec::with_capacity(groups.len());
    for &g in groups {
        let mut min_index = i32::MAX;
        let sqrt_g = (g as f64).sqrt() as i32;
        for j in 1..=sqrt_g {
            if g % j != 0 {
                continue;
            }
            let k = g / j;
            let j_val = index[j as usize];
            let k_val = index[k as usize];
            // Check if either factor is present in the elements array.
            if j_val != i32::MAX || k_val != i32::MAX {
                let current_min = j_val.min(k_val);
                if current_min < min_index {
                    min_index = current_min;
                }
            }
        }
        // If no valid index found, set to -1.
        ans.push(if min_index == i32::MAX { -1 } else { min_index });
    }
    ans
}