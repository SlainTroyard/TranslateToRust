// Problem: Weekly Contest 417 Problem 4
use std::io;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn kth_character(&self, k: i64, operations: &Vec<i32>) -> char {
        let mut k_mut = k - 1;
        let mut inc = 0;
        let lg_k = if k_mut == -1 { -1 } else { 63 - k_mut.leading_zeros() as i32 };
        for i in (0..=lg_k).rev() {
            if (k_mut >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }
        ('a' as u8 + (inc % 26) as u8) as char
    }
}

fn read_line<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().parse()
}

fn read_vec<T: FromStr>(n: usize) -> Result<Vec<T>, <T as FromStr>::Err> {
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(read_line()?);
    }
    Ok(vec)
}

fn main() {
    let line = read_line::<String>().unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();
    let k: i64 = parts[0].parse().unwrap();
    let operation_size: usize = parts[1].parse().unwrap();

    let operations: Vec<i32> = read_vec(operation_size).unwrap();

    let s = Solution {};
    println!("{}", s.kth_character(k, &operations));
}