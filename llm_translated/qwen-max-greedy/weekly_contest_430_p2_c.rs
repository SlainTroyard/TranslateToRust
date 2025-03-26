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
        if word[i..].get(0..n).unwrap_or(&"") > word[ans..].get(0..n).unwrap_or(&"") {
            ans = i;
        }
    }
    if ans + n < len {
        &word[ans..ans + n]
    } else {
        &word[ans..]
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    // Input word and numFriends
    println!("Enter the word:");
    io::stdin().read_line(&mut input)?;
    let word = input.trim();

    println!("Enter the number of friends:");
    input.clear();
    io::stdin().read_line(&mut input)?;
    let num_friends: usize = input.trim().parse().expect("Please type a number!");

    // Call answer_string function
    let result = answer_string(word, num_friends);

    // Print the result
    println!("{}", result);
    Ok(())
}