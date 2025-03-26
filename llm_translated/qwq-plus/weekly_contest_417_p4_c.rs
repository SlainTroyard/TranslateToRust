use std::io;

fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    if pos == 0 || k == 1 {
        if operations[pos] != 0 {
            'b'
        } else {
            'a'
        }
    } else {
        let mut pow_sum = 1;
        let mut tmp_pos = 0;
        while k > pow_sum {
            pow_sum *= 2;
            tmp_pos += 1;
        }
        let next_k = k - pow_sum / 2;
        let next_pos = tmp_pos - 1;
        if operations[pos] != 0 {
            let mut c = kchar_search(next_k, operations, next_pos);
            c = (c as u8 + 1) as char;
            if c > 'z' {
                'a'
            } else {
                c
            }
        } else {
            kchar_search(next_k, operations, next_pos)
        }
    }
}

fn kth_character(k: i64, operations: &[i32]) -> char {
    if k == 1 {
        return 'a';
    }
    let mut pow_sum = 1;
    let mut pos = 0;
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }
    let adjusted_k = k - pow_sum / 2;
    let adjusted_pos = pos - 1;
    kchar_search(adjusted_k, operations, adjusted_pos)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let k = tokens[0].parse::<i64>().unwrap();
    let operations_size = tokens[1].parse::<usize>().unwrap();
    let operations: Vec<i32> = tokens[2..2 + operations_size]
        .iter()
        .map(|&s| s.parse().unwrap())
        .collect();
    let result = kth_character(k, &operations);
    println!("{}", result);
}