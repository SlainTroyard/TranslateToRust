use std::io::{self, Read};

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    const MAX_ELEMENT: usize = 100_004; // 0-based index up to 100_004 (size 100_005)
    let mut index = vec![i32::MAX; MAX_ELEMENT + 1]; // Initialize with i32::MAX

    // Populate index array with the earliest occurrence of each element
    for (i, &element) in elements.iter().enumerate() {
        if element >= 0 && (element as usize) <= MAX_ELEMENT {
            let e = element as usize;
            index[e] = index[e].min(i as i32);
        }
    }

    groups.iter().map(|&g| {
        let mut min_idx = i32::MAX;
        if g == 0 {
            // No divisors other than 0 (handled as invalid)
            return -1;
        }

        let sqrt_g = (g.abs() as f64).sqrt().floor() as i32;
        for j in 1..=sqrt_g {
            if g % j != 0 {
                continue;
            }

            // Check both j and g/j as divisors
            let d1 = j;
            let d2 = g / j;

            let idx1 = if d1 >= 0 && (d1 as usize) <= MAX_ELEMENT {
                index[d1 as usize]
            } else {
                i32::MAX
            };

            let idx2 = if d2 >= 0 && (d2 as usize) <= MAX_ELEMENT {
                index[d2 as usize]
            } else {
                i32::MAX
            };

            let current_min = idx1.min(idx2);
            if current_min != i32::MAX {
                min_idx = min_idx.min(current_min);
            }
        }

        if min_idx == i32::MAX {
            -1
        } else {
            min_idx
        }
    }).collect()
}

fn main() {
    // Read entire input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer in input"));

    // Parse n and m
    let n = tokens.next().expect("Missing n");
    let m = tokens.next().expect("Missing m");

    // Read groups
    let mut groups = Vec::with_capacity(n as usize);
    for _ in 0..n {
        groups.push(tokens.next().expect("Insufficient group data"));
    }

    // Read elements
    let mut elements = Vec::with_capacity(m as usize);
    for _ in 0..m {
        elements.push(tokens.next().expect("Insufficient element data"));
    }

    // Compute answer
    let ans = assign_elements(&groups, &elements);

    // Print result with space-separated integers followed by newline
    for (i, &num) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}