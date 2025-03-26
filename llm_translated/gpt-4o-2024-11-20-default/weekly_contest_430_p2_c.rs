use std::io::{self, Write};

fn answer_string(word: &mut String, num_friends: usize) -> String {
    if num_friends == 1 {
        return word.clone();
    }

    let len = word.len();
    let n = len - num_friends + 1;
    let mut ans = 0;

    let bytes = word.as_bytes(); // Work with the byte representation of the string
    for i in 0..len {
        if bytes[i..].iter().take(n).cmp(bytes[ans..].iter().take(n)) == std::cmp::Ordering::Greater {
            ans = i;
        }
    }

    if ans + n < len {
        word.truncate(ans + n); // Truncate the original string
    }
    word[ans..].to_string() // Return the substring starting from `ans`
}

fn main() {
    let mut input = String::new();

    // Read 'word' from stdin
    io::stdin().read_line(&mut input).expect("Failed to read word");
    let mut word = input.trim().to_string(); // Ensure no extraneous newlines or spaces

    // Read 'numFriends' from stdin
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read numFriends");
    let num_friends: usize = input.trim().parse().expect("Failed to parse numFriends");

    // Call `answer_string` function
    let result = answer_string(&mut word, num_friends);

    // Display the result
    println!("{}", result);
}