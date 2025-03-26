use std::cmp::Ordering;
use std::io::{self, Write};

// Global variable to store the length of each segment
static mut LEN: usize = 0;

// Custom comparator function for sorting segments
fn cmp(a: &[u8], b: &[u8]) -> Ordering {
    unsafe {
        a[..LEN].cmp(&b[..LEN])
    }
}

// Function to check if it's possible to rearrange the strings
fn is_possible_to_rearrange(s: &mut [u8], t: &mut [u8], k: usize) -> bool {
    unsafe {
        LEN = s.len() / k;
        s.chunks_mut(LEN).for_each(|chunk| chunk.sort());
        t.chunks_mut(LEN).for_each(|chunk| chunk.sort());
        s == t
    }
}

fn main() {
    // Read input strings and integer k
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let mut s = s.trim().as_bytes().to_vec();

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let mut t = t.trim().as_bytes().to_vec();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: usize = k_str.trim().parse().expect("Failed to parse k");

    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(&mut s, &mut t, k) {
        println!("true");
    } else {
        println!("false");
    }
}