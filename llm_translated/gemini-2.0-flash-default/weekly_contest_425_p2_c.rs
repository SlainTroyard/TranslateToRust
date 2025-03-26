use std::cmp::Ordering;
use std::io;

static mut LEN: usize = 0;

fn is_possible_to_rearrange(s: &mut [u8], t: &mut [u8], k: usize) -> bool {
    unsafe {
        LEN = s.len() / k;
    }

    // Define a comparison function for sorting substrings
    fn compare_substrings(a: &[u8], b: &[u8], len: usize) -> Ordering {
        let a_str = String::from_utf8_lossy(a);
        let b_str = String::from_utf8_lossy(b);

        let a_sub = &a_str[..len];
        let b_sub = &b_str[..len];
        a_sub.cmp(b_sub)
    }

    // Sort the substrings in s
    let len = unsafe { LEN };
    s.chunks_mut(len).sort_by(|a, b| compare_substrings(a, b, len));

    // Sort the substrings in t
    t.chunks_mut(len).sort_by(|a, b| compare_substrings(a, b, len));

    // Compare the rearranged strings
    s == t
}

fn main() {
    // Read input from stdin
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