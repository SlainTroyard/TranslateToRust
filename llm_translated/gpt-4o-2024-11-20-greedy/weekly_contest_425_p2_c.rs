use std::cmp::Ordering;
use std::io::{self, BufRead};

fn cmp(a: &&str, b: &&str, len: usize) -> Ordering {
    a[..len].cmp(&b[..len])
}

fn is_possible_to_rearrange(s: &mut Vec<String>, t: &mut Vec<String>, k: usize) -> bool {
    let len = s[0].len() / k;

    // Sort the chunks of `s` and `t` based on the first `len` characters
    s.sort_by(|a, b| cmp(a, b, len));
    t.sort_by(|a, b| cmp(a, b, len));

    // Compare the sorted chunks
    s == t
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first string `s`
    let s = lines.next().unwrap().unwrap();

    // Read the second string `t`
    let t = lines.next().unwrap().unwrap();

    // Read the integer `k`
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Split the strings into chunks of size `LEN`
    let len = s.len() / k;
    let mut s_chunks: Vec<String> = s.chars().collect::<Vec<_>>()
        .chunks(len)
        .map(|chunk| chunk.iter().collect())
        .collect();
    let mut t_chunks: Vec<String> = t.chars().collect::<Vec<_>>()
        .chunks(len)
        .map(|chunk| chunk.iter().collect())
        .collect();

    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(&mut s_chunks, &mut t_chunks, k) {
        println!("true");
    } else {
        println!("false");
    }
}