use std::cmp::Ordering;
use std::io::{self, Write};

fn cmp(a: &&[u8], b: &&[u8], len: usize) -> Ordering {
    a[..len].cmp(&b[..len])
}

fn is_possible_to_rearrange(s: &mut [u8], t: &mut [u8], k: usize) -> bool {
    let len = s.len() / k;
    let mut s_chunks: Vec<&[u8]> = s.chunks_exact(len).collect();
    let mut t_chunks: Vec<&[u8]> = t.chunks_exact(len).collect();
    
    // Sort the groups by their lexicographical order
    s_chunks.sort_by(|a, b| cmp(a, b, len));
    t_chunks.sort_by(|a, b| cmp(a, b, len));

    // Compare the sorted groups
    s_chunks == t_chunks
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read input from stdin. The input format consists of three lines:
    // Line 1: The string `s`
    // Line 2: The string `t`
    // Line 3: The integer `k`
    
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    let mut s = input.trim_end().as_bytes().to_vec();
    
    input.clear();
    stdin.read_line(&mut input)?;
    let mut t = input.trim_end().as_bytes().to_vec();
    
    input.clear();
    stdin.read_line(&mut input)?;
    let k: usize = input.trim_end().parse().expect("Expected an integer");

    // Check if the strings `s` and `t` can be rearranged and sorted in groups of size `LEN`
    if is_possible_to_rearrange(&mut s, &mut t, k) {
        writeln!(stdout, "true")?;
    } else {
        writeln!(stdout, "false")?;
    }

    Ok(())
}