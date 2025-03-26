use std::io::{self, Read};

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    if k == 0 {
        return false;
    }
    
    if s.len() != t.len() {
        return false;
    }
    
    let len = s.len() / k;
    
    let mut s_bytes = s.as_bytes().to_vec();
    let mut t_bytes = t.as_bytes().to_vec();
    
    process_bytes(&mut s_bytes, k, len);
    process_bytes(&mut t_bytes, k, len);
    
    s_bytes == t_bytes
}

fn process_bytes(bytes: &mut Vec<u8>, k: usize, len: usize) {
    if len == 0 {
        return;
    }
    
    let total_processed = k * len;
    let processed_len = total_processed.min(bytes.len());
    let processed = &mut bytes[..processed_len];
    
    let mut chunks: Vec<&[u8]> = processed.chunks(len).collect();
    chunks.truncate(k);
    
    chunks.sort_unstable();
    
    let sorted: Vec<u8> = chunks.into_iter().flatten().copied().collect();
    let copy_len = sorted.len().min(processed.len());
    processed[..copy_len].copy_from_slice(&sorted[..copy_len]);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    let s = tokens.next().unwrap_or("");
    let t = tokens.next().unwrap_or("");
    let k: usize = tokens.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    
    println!("{}", if is_possible_to_rearrange(s, t, k) { "true" } else { "false" });
}