use std::io::{self, BufRead};

/// Finds the lexicographically largest substring of the word with a specific length.
/// 
/// This function looks for the best substring with length `len - numFriends + 1`
/// where 'best' means lexicographically largest.
///
/// # Arguments
/// * `word` - The original word
/// * `numFriends` - The number of friends (used to calculate substring length)
fn answer_string(word: &str, num_friends: i32) -> String {
    if num_friends == 1 {
        return word.to_string();
    }
    
    let len = word.len();
    let n = len - num_friends as usize + 1;
    let mut ans = 0;
    
    for i in 0..len {
        // Make sure we don't go out of bounds
        if i + n <= len {
            // Compare the substring starting at i with the current best substring
            if word[i..].cmp(&word[ans..]).is_gt() {
                ans = i;
            }
        }
    }
    
    // Return the substring starting at ans with length n
    word[ans..].chars().take(n).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the word
    let word = lines.next().unwrap().unwrap();
    
    // Read the number of friends
    let num_friends: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Call the answer_string function
    let result = answer_string(&word, num_friends);
    
    // Print the result
    println!("{}", result);
}