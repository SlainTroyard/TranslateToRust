use std::io::{self, BufRead};

/// Finds the lexicographically largest substring of length (len - numFriends + 1)
/// 
/// # Arguments
/// * `word` - The input string
/// * `num_friends` - Number of friends
/// 
/// # Returns
/// * The lexicographically largest substring
fn answer_string(word: &str, num_friends: i32) -> String {
    if num_friends == 1 {
        return word.to_string();
    }
    
    let len = word.len();
    let n = len - num_friends as usize + 1;
    let mut ans = 0;
    
    for i in 0..len {
        if i + n <= len && word[i..].cmp(&word[ans..]).is_gt() {
            ans = i;
        }
    }
    
    word[ans..ans+n].to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read word
    let word = lines.next().unwrap().unwrap();
    
    // Read numFriends
    let num_friends: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Call answer_string function
    let result = answer_string(&word, num_friends);
    
    // Print the result
    println!("{}", result);
}