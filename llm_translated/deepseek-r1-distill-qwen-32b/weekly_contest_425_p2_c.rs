fn is_possible_to_rearrange(s: String, t: String, k: usize) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let len = s.len() / k;
    if s.len() % k != 0 || t.len() % k != 0 {
        return false;
    }
    
    // Split s into chunks
    let s_chunks = (0..k)
        .map(|i| {
            let start = i * len;
            let end = start + len;
            s[start..end].to_string()
        })
        .collect::<Vec<String>>();
    
    // Split t into chunks
    let t_chunks = (0..k)
        .map(|i| {
            let start = i * len;
            let end = start + len;
            t[start..end].to_string()
        })
        .collect::<Vec<String>>();
    
    // Sort the chunks
    let mut sorted_s = s_chunks.clone();
    sorted_s.sort();
    
    let mut sorted_t = t_chunks.clone();
    sorted_t.sort();
    
    // Concatenate and compare
    sorted_s.join("") == sorted_t.join("")
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();
    let mut k = String::new();
    
    std::io::stdin().read_line(&mut s).unwrap();
    std::io::stdin().read_line(&mut t).unwrap();
    std::io::stdin().read_line(&mut k).unwrap();
    
    s = s.trim().to_string();
    t = t.trim().to_string();
    let k: usize = k.trim().parse().unwrap();
    
    if is_possible_to_rearrange(s, t, k) {
        println!("true");
    } else {
        println!("false");
    }
}