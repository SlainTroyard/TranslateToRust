use std::cmp::Ordering;

// Helper function to compare two byte slices up to 'n' bytes, treating missing bytes as 0
fn compare_slices(a: &[u8], b: &[u8], n: usize) -> Ordering {
    for i in 0..n {
        let byte_a = a.get(i).copied().unwrap_or(0);
        let byte_b = b.get(i).copied().unwrap_or(0);
        match byte_a.cmp(&byte_b) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

fn answer_string(word: &str, num_friends: usize) -> String {
    if num_friends == 1 {
        return word.to_string();
    }
    let len = word.len();
    let n = len - num_friends + 1;
    let bytes = word.as_bytes();
    let mut ans = 0;

    // Find the starting index with the lexicographically largest substring
    for i in 0..len {
        let current_slice = &bytes[i..];
        let best_slice = &bytes[ans..];
        if compare_slices(current_slice, best_slice, n) == Ordering::Greater {
            ans = i;
        }
    }

    // Calculate the end index, ensuring we don't exceed the string bounds
    let end = (ans + n).min(len);
    let result_bytes = &bytes[ans..end];
    
    // Convert bytes back to string (input is valid UTF-8 per original code)
    String::from_utf8(result_bytes.to_vec()).unwrap()
}

fn main() {
    // Read entire input and split into tokens to handle multiple input formats
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse input exactly like original C code's scanf("%s %d") pattern
    let word = tokens.next().expect("No word provided");
    let num_friends: usize = tokens.next()
        .expect("No num_friends provided")
        .parse()
        .expect("Invalid num_friends");

    let result = answer_string(word, num_friends);
    println!("{}", result);
}