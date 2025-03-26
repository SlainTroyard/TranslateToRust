use std::cmp::Ordering;
use std::io::{self, Read};

fn is_possible_to_rearrange(s: &mut String, t: &mut String, k: usize) -> bool {
    let len = s.len() / k;
    
    // Split s and t into k chunks
    let mut s_chunks: Vec<String> = Vec::with_capacity(k);
    let mut t_chunks: Vec<String> = Vec::with_capacity(k);
    
    for i in 0..k {
        let start = i * len;
        let end = start + len;
        s_chunks.push(s[start..end].to_string());
        t_chunks.push(t[start..end].to_string());
    }
    
    // Sort chunks
    s_chunks.sort();
    t_chunks.sort();
    
    // Recombine chunks
    let mut new_s = String::with_capacity(s.len());
    let mut new_t = String::with_capacity(t.len());
    
    for chunk in s_chunks {
        new_s.push_str(&chunk);
    }
    
    for chunk in t_chunks {
        new_t.push_str(&chunk);
    }
    
    // Compare the sorted strings
    new_s == new_t
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let mut tokens = input.split_whitespace();
    
    // Parse the input
    let s = tokens.next().unwrap().to_string();
    let t = tokens.next().unwrap().to_string();
    let k: usize = tokens.next().unwrap().parse().expect("Failed to parse k");
    
    // Check if rearrangement is possible
    let result = is_possible_to_rearrange(&mut s.clone(), &mut t.clone(), k);
    
    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}