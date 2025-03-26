use std::io::Read;

fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    if pos == 0 || k == 1 {
        return if operations[pos] != 0 { 'b' } else { 'a' };
    }

    let mut pow_sum = 1i64;
    let mut tmp_pos = 0;

    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    let new_k = k - pow_sum / 2;
    let next_pos = tmp_pos - 1;

    if operations[pos] != 0 {
        let c = kchar_search(new_k, operations, next_pos);
        if c == 'z' {
            'a'
        } else {
            ((c as u8) + 1) as char
        }
    } else {
        kchar_search(new_k, operations, next_pos)
    }
}

fn kth_character(k: i64, operations: &[i32]) -> char {
    if k == 1 {
        return 'a';
    }

    let mut pow_sum = 1i64;
    let mut pos = 0;

    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    let new_k = k - pow_sum / 2;
    kchar_search(new_k, operations, pos - 1)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let k: i64 = tokens
        .next()
        .expect("Missing k")
        .parse()
        .expect("Invalid k");
    let operation_size: usize = tokens
        .next()
        .expect("Missing operation size")
        .parse()
        .expect("Invalid operation size");

    let operations: Vec<i32> = tokens
        .take(operation_size)
        .map(|s| s.parse().expect("Invalid operation"))
        .collect();

    assert_eq!(
        operations.len(),
        operation_size,
        "Incorrect number of operations provided"
    );

    let result = kth_character(k, &operations);
    println!("{}", result);
}