use std::io::{self, BufRead};
use std::cmp::Ordering;

fn is_possible_to_rearrange(s: &mut String, t: &mut String, k: usize) -> bool {
    let len = s.len() / k;
    
    // Convert strings to vectors of substrings for sorting
    let mut s_chunks: Vec<String> = (0..k).map(|i| {
        s.chars().skip(i * len).take(len).collect()
    }).collect();
    
    let mut t_chunks: Vec<String> = (0..k).map(|i| {
        t.chars().skip(i * len).take(len).collect()
    }).collect();
    
    // Sort the chunks
    s_chunks.sort();
    t_chunks.sort();
    
    // Rebuild the sorted strings
    *s = s_chunks.join("");
    *t = t_chunks.join("");
    
    // Compare the sorted strings
    s == t
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input strings and k
    let mut s = lines.next().unwrap().unwrap();
    let mut t = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(&mut s, &mut t, k) {
        println!("true");
    } else {
        println!("false");
    }
}