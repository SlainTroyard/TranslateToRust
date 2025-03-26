use std::cmp::Ordering;
use std::io;

// Global variable to store LEN, initialized outside the function
static mut LEN: usize = 0;

// Function to compare two strings up to LEN bytes
fn cmp(a: &[u8], b: &[u8]) -> Ordering {
    unsafe {
        let len = LEN;
        a[..len].cmp(&b[..len])
    }
}

fn is_possible_to_rearrange(s: &mut [u8], t: &mut [u8], k: usize) -> bool {
    unsafe {
        LEN = s.len() / k;
    }

    let len_val = unsafe { LEN };

    // Sort s
    s.chunks_mut(len_val)
        .collect::<Vec<_>>()
        .sort_by(|a, b| cmp(a, b));

    // Sort t
    t.chunks_mut(len_val)
        .collect::<Vec<_>>()
        .sort_by(|a, b| cmp(a, b));

    s == t
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut s = s.trim().as_bytes().to_vec();

    let mut t = String::new();
    io::stdin().read_line(&mut t)?;
    let mut t = t.trim().as_bytes().to_vec();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str)?;
    let k: usize = k_str.trim().parse().unwrap();

    if is_possible_to_rearrange(&mut s, &mut t, k) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}