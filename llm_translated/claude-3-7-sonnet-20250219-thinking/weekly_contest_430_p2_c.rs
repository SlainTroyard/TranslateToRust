use std::io::{self, BufRead};

fn answer_string(word: &str, num_friends: i32) -> String {
    if num_friends == 1 {
        return word.to_string();
    }
    
    let len = word.len();
    let n = len - num_friends as usize + 1;
    let mut ans = 0;
    
    for i in 0..len {
        // Compare substrings starting at position i and ans
        // Only comparing first n characters (same as strncmp)
        let end_i = std::cmp::min(i + n, len);
        let end_ans = std::cmp::min(ans + n, len);
        
        if word[i..end_i] > word[ans..end_ans] {
            ans = i;
        }
    }
    
    // Return the substring starting at ans with at most n characters
    word[ans..std::cmp::min(ans+n, len)].to_string()
}

fn main() {
    // Read word from stdin
    let mut word = String::new();
    io::stdin().lock().read_line(&mut word).expect("Failed to read word");
    let word = word.trim();
    
    // Read numFriends from stdin
    let mut num_friends = String::new();
    io::stdin().lock().read_line(&mut num_friends).expect("Failed to read numFriends");
    let num_friends: i32 = num_friends.trim().parse().expect("Failed to parse numFriends");
    
    // Call the function and print result
    let result = answer_string(word, num_friends);
    println!("{}", result);
}