use std::io::{self, BufRead};
use std::str::FromStr;

fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;

    // Base case: if pos is 0 or k is 1
    if pos == 0 || k == 1 {
        if operations[pos] != 0 {
            return 'b';
        }
        return 'a';
    }

    // Find the position where pow_sum exceeds k
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

fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;

    // Base case: if k is 1
    if k == 1 {
        return 'a';
    }

    // Find the position where pow_sum exceeds k
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and operationSize from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let k = i64::from_str(first_line_iter.next().unwrap()).unwrap();
    let operation_size = usize::from_str(first_line_iter.next().unwrap()).unwrap();

    // Read the operations array from the second line
    let second_line = lines.next().unwrap().unwrap();
    let operations: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| i32::from_str(x).unwrap())
        .collect();

    // Ensure the operations array size matches the input
    assert_eq!(operations.len(), operation_size);

    // Compute the kth character and print it
    let result = kth_character(k, &operations);
    println!("{}", result);
}