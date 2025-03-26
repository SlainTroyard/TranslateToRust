use std::cmp::Ordering;
use std::io::{self, BufRead};

// Global variable to store the length of each segment
static mut LEN: usize = 0;

// Custom comparison function for sorting
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line as string s
    let s = lines.next().unwrap().unwrap();
    // Read the second line as string t
    let t = lines.next().unwrap().unwrap();
    // Read the third line as integer k
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Convert strings to mutable byte arrays
    let mut s_bytes = s.into_bytes();
    let mut t_bytes = t.into_bytes();

    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(&mut s_bytes, &mut t_bytes, k) {
        println!("true");
    } else {
        println!("false");
    }
}