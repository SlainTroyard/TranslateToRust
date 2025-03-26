use std::io::{self, BufRead};

// Function to find the k-th character based on operations
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;
    if pos == 0 || k == 1 {
        if operations[pos] != 0 {
            return 'b';
        }
        return 'a';
    }

    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    if operations[pos] != 0 {
        let mut kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        kchar = ((kchar as u8) + 1) as char;
        if kchar > 'z' {
            return 'a';
        }
        return kchar;
    }

    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

// Function to find the k-th character after applying operations
fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;

    if k == 1 {
        return 'a';
    }

    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and operation size
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let k: i64 = parts.next().unwrap().parse().unwrap();
    let operation_size: usize = parts.next().unwrap().parse().unwrap();

    // Read operations
    let operations: Vec<i32> = lines
        .take(operation_size)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = kth_character(k, &operations);
    println!("{}", result);

    Ok(())
}