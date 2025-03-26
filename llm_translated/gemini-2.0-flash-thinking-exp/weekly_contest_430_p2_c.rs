use std::io;
use std::cmp::Ordering;

fn answer_string(word: &mut String, num_friends: i32) -> &str {
    if num_friends == 1 {
        return word;
    }
    let len = word.len();
    let n = len - num_friends as usize + 1;
    let mut ans = 0;
    for i in 1..=len - n {
        let slice1 = &word[i..i + n];
        let slice2 = &word[ans..ans + n];
        if slice1.cmp(slice2) == Ordering::Greater {
            ans = i;
        }
    }
    // In C, the original code modifies the input string by null-terminating it.
    // This is not idiomatic in Rust, and we will just return a slice.
    // The null termination part in C:
    // if (ans + n < len)
    //     *(word + ans + n) = '\0';
    // return word + ans;
    &word[ans..]
}

fn main() {
    let mut word = String::new();
    let mut num_friends_str = String::new();

    // Input word
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = &mut word.trim_end().to_string(); // Remove trailing newline

    // Input numFriends
    io::stdin().read_line(&mut num_friends_str).expect("Failed to read line");
    let num_friends: i32 = num_friends_str.trim().parse().expect("Invalid input for numFriends");

    // Call answerString function
    let result = answer_string(word, num_friends);

    // Print the result
    println!("{}", result);
}