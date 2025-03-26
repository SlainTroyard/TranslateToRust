use std::io::{self, BufRead};
use std::cmp::Ordering;

fn is_possible_to_rearrange(s: &mut String, t: &mut String, k: usize) -> bool {
    let len = s.len() / k;
    
    // Convert strings to character vectors for easier manipulation
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    
    // Create vectors of k substrings each of length 'len'
    let mut s_substrings: Vec<String> = Vec::with_capacity(k);
    let mut t_substrings: Vec<String> = Vec::with_capacity(k);
    
    for i in 0..k {
        let start = i * len;
        let end = start + len;
        let s_substring: String = s_chars[start..end].iter().collect();
        let t_substring: String = t_chars[start..end].iter().collect();
        
        s_substrings.push(s_substring);
        t_substrings.push(t_substring);
    }
    
    // Sort the substrings
    s_substrings.sort();
    t_substrings.sort();
    
    // Join all substrings to form the complete strings
    let sorted_s: String = s_substrings.join("");
    let sorted_t: String = t_substrings.join("");
    
    // Compare the sorted strings
    sorted_s == sorted_t
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input strings and value k
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Check if it is possible to rearrange the strings
    let mut s_mut = s.clone();
    let mut t_mut = t.clone();
    
    if is_possible_to_rearrange(&mut s_mut, &mut t_mut, k) {
        println!("true");
    } else {
        println!("false");
    }
}