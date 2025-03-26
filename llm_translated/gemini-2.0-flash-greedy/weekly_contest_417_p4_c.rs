use std::io;
use std::io::Read;

fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum: i64 = 1;
    let mut tmp_pos: usize = 0;

    if pos == 0 && k == 1 {
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
        kchar = (kchar as u8 + 1) as char;
        if kchar > 'z' {
            return 'a';
        }
        return kchar;
    }

    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut pow_sum: i64 = 1;
    let mut pos: usize = 0;

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let k: i64 = first_split.next().unwrap().parse().unwrap();
    let operation_size: usize = first_split.next().unwrap().parse().unwrap();

    let mut operations: Vec<i32> = Vec::with_capacity(operation_size);
    for _ in 0..operation_size {
        let line = lines.next().unwrap();
        let operation: i32 = line.parse().unwrap();
        operations.push(operation);
    }

    println!("{}", kth_character(k, &operations));

    Ok(())
}