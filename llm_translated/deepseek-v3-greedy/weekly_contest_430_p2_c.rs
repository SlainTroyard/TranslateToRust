use std::io::{self, Write};

// Function to find the answer string based on the given word and number of friends
fn answer_string(word: &mut String, num_friends: usize) -> &str {
    if num_friends == 1 {
        return word;
    }
    let len = word.len();
    let n = len - num_friends + 1;
    let mut ans = 0;

    // Iterate through the word to find the starting index of the largest substring
    for i in 0..len {
        if word[i..].cmp(&word[ans..]).then(word[i..].len().cmp(&word[ans..].len())) == std::cmp::Ordering::Greater {
            ans = i;
        }
    }

    // Truncate the word to the selected substring if necessary
    if ans + n < len {
        word.truncate(ans + n);
    }

    &word[ans..]
}

fn main() {
    let mut word = String::new();
    let mut num_friends = String::new();

    // Read the input word
    io::stdin().read_line(&mut word).expect("Failed to read word");
    word = word.trim().to_string();

    // Read the number of friends
    io::stdin().read_line(&mut num_friends).expect("Failed to read number of friends");
    let num_friends: usize = num_friends.trim().parse().expect("Invalid number of friends");

    // Call the answer_string function
    let result = answer_string(&mut word, num_friends);

    // Print the result
    println!("{}", result);
}