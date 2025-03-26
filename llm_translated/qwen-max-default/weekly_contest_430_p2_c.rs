// Problem: Weekly Contest 430 Problem 2
use std::io;
use std::cmp::Ordering;

fn answer_string(word: &str, num_friends: usize) -> &str {
    if num_friends == 1 {
        return word;
    }
    let len = word.len();
    let n = len - num_friends + 1;
    let mut ans = 0;
    for i in 0..len {
        if word[i..].cmp(&word[ans..]) == Ordering::Greater {
            ans = i;
        }
    }
    if ans + n < len {
        &word[ans..ans + n]
    } else {
        &word[ans..]
    }
}

fn main() {
    // Input word and numFriends
    let mut word = String::new();
    let mut num_friends_str = String::new();

    println!("Enter the word:");
    io::stdin().read_line(&mut word).expect("Failed to read line");
    word = word.trim().to_string(); // Remove any trailing newline or whitespace

    println!("Enter the number of friends:");
    io::stdin().read_line(&mut num_friends_str).expect("Failed to read line");
    let num_friends: usize = num_friends_str.trim().parse().expect("Please type a number!");

    // Call answer_string function
    let result = answer_string(&word, num_friends);

    // Print the result
    println!("{}", result);
}