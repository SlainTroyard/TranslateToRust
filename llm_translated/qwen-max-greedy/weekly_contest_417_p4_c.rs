use std::io;

fn kchar_search(k: usize, operations: &[i32], pos: usize) -> char {
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
        let kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        if kchar as u8 + 1 > b'z' {
            return 'a';
        }
        return (kchar as u8 + 1) as char;
    }

    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

fn kth_character(k: usize, operations: &[i32]) -> char {
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

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_parts: Vec<&str> = input.trim().split_whitespace().collect();
    let k: usize = input_parts[0].parse().expect("Invalid input for k");
    let operation_size: usize = input_parts[1].parse().expect("Invalid input for operation size");

    // Allocate and read operations
    let mut operations: Vec<i32> = vec![0; operation_size];
    for i in 0..operation_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        operations[i] = input.trim().parse().expect("Invalid input for operations");
    }

    // Compute and print the result
    let result = kth_character(k, &operations);
    println!("{}", result);
}