use std::io::{self, Read};

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Initialize diff vector with 26 zeroes.
    // For each character in word2, decrement the corresponding count.
    let mut diff = vec![0i32; 26];
    for b in word2.bytes() {
        diff[(b - b'a') as usize] -= 1;
    }

    // Count how many characters have negative counts.
    let mut cnt = diff.iter().filter(|&&c| c < 0).count() as i32;
    let n = word1.len();
    let word1_bytes = word1.as_bytes();
    let mut res: i64 = 0;

    // Define the update closure:
    // For the character represented by its index, update its diff value.
    // If add == 1 and the new count becomes 0, it means we fixed a deficit, so decrement cnt.
    // If add == -1 and the new count becomes -1, it means we reintroduced a deficit, so increment cnt.
    let mut update = |c_index: usize, add: i32| {
        diff[c_index] += add;
        if add == 1 && diff[c_index] == 0 {
            cnt -= 1;
        } else if add == -1 && diff[c_index] == -1 {
            cnt += 1;
        }
    };

    let mut r = 0;
    // Slide the window over word1 using two pointers.
    for l in 0..n {
        // Expand the window until all deficits are resolved (i.e., cnt becomes 0)
        while r < n && cnt > 0 {
            let c_index = (word1_bytes[r] - b'a') as usize;
            update(c_index, 1);
            r += 1;
        }
        // If all deficits are resolved, any further extension of the window will be valid.
        if cnt == 0 {
            // The count of valid substrings starting at l is: n - r + 1.
            res += (n - r + 1) as i64;
        }
        // Shrink the window from the left for the next iteration.
        let l_index = (word1_bytes[l] - b'a') as usize;
        update(l_index, -1);
    }
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire input from stdin into a String.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split input by whitespace.
    let mut tokens = input.split_whitespace();
    
    // Read len1 (although we don't actually need it for processing, but we read it to stay in sync with the input format)
    let _len1: usize = tokens
        .next()
        .ok_or("Missing len1")?
        .parse()
        .map_err(|_| "Invalid len1")?;
    
    // Read word1
    let word1 = tokens.next().ok_or("Missing word1")?;
    
    // Read len2
    let _len2: usize = tokens
        .next()
        .ok_or("Missing len2")?
        .parse()
        .map_err(|_| "Invalid len2")?;
    
    // Read word2
    let word2 = tokens.next().ok_or("Missing word2")?;
    
    // Compute the valid substring count.
    let result = valid_substring_count(word1, word2);
    
    // Output the result exactly as the original program
    println!("{}", result);
    
    Ok(())
}