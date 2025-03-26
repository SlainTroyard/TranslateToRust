use std::io::{self, Read};

// Global variable to store the length of each substring
static mut LEN: usize = 0;

// Custom comparison function for sorting
fn cmp(a: &str, b: &str) -> std::cmp::Ordering {
    unsafe {
        a.chars().take(LEN).cmp(b.chars().take(LEN))
    }
}

// Function to check if it's possible to rearrange the strings
fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    unsafe {
        LEN = s.len() / k;
        let mut s_vec: Vec<&str> = s.as_bytes().chunks(LEN).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();
        let mut t_vec: Vec<&str> = t.as_bytes().chunks(LEN).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();

        s_vec.sort_by(cmp);
        t_vec.sort_by(cmp);

        s_vec.join("") == t_vec.join("")
    }
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse input
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let t = lines.next().unwrap().trim();
    let k: usize = lines.next().unwrap().trim().parse().unwrap();

    // Check if it's possible to rearrange the strings
    if is_possible_to_rearrange(s, t, k) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}