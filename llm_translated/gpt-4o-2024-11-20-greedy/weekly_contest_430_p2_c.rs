use std::io::{self, Write};

fn answer_string(word: &mut String, num_friends: usize) -> &str {
    if num_friends == 1 {
        return word;
    }

    let len = word.len();
    let n = len - num_friends + 1;
    let mut ans = 0;

    for i in 0..len {
        if word[i..].chars().take(n).cmp(word[ans..].chars().take(n)) == std::cmp::Ordering::Greater {
            ans = i;
        }
    }

    if ans + n < len {
        word.truncate(ans + n);
    }

    &word[ans..]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let word = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_friends: usize = input.trim().parse().expect("Failed to parse numFriends");

    let mut word = word.clone(); // Create a mutable copy of the word
    let result = answer_string(&mut word, num_friends);

    println!("{}", result);
}