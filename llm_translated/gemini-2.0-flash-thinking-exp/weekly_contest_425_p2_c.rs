// Problem: Weekly Contest 425 Problem 2
use std::io;
use std::cmp::Ordering;

fn is_possible_to_rearrange(s: &mut str, t: &mut str, k: i32) -> bool {
    let len = s.len() / (k as usize);

    // Define the comparison function for sorting substrings
    let cmp = |a: &[u8], b: &[u8]| -> Ordering {
        let len_cmp = len.min(a.len()).min(b.len());
        for i in 0..len_cmp {
            match a[i].cmp(&b[i]) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => continue,
            }
        }
        Ordering::Equal // If prefixes are equal up to len, consider them equal
    };

    // Convert strings to mutable byte slices for sorting
    let s_bytes = s.as_bytes_mut();
    let t_bytes = t.as_bytes_mut();

    // Sort substrings of s
    let mut s_substrings: Vec<&mut [u8]> = s_bytes.chunks_mut(len).collect();
    s_substrings.sort_by(|a, b| cmp(a, b));
    let mut s_sorted_bytes: Vec<u8> = Vec::new();
    for sub in s_substrings.iter_mut() {
        s_sorted_bytes.extend_from_slice(sub);
    }
    // Modify s in place
    for i in 0..s_bytes.len() {
        s_bytes[i] = s_sorted_bytes[i];
    }


    // Sort substrings of t
    let mut t_substrings: Vec<&mut [u8]> = t_bytes.chunks_mut(len).collect();
    t_substrings.sort_by(|a, b| cmp(a, b));
    let mut t_sorted_bytes: Vec<u8> = Vec::new();
    for sub in t_substrings.iter_mut() {
        t_sorted_bytes.extend_from_slice(sub);
    }
    // Modify t in place
    for i in 0..t_bytes.len() {
        t_bytes[i] = t_sorted_bytes[i];
    }


    s == t
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let mut s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let mut t = t.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k");

    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(&mut s, &mut t, k) {
        println!("true");
    } else {
        println!("false");
    }
}