use std::io::{self, BufRead, Write};

fn answer_string(word: &str, num_friends: usize) -> &str {
    // If there's only one friend, return the entire word unchanged.
    if num_friends == 1 {
        return word;
    }

    // Get the number of bytes in the input word.
    let len = word.len();
    // Compute n as in C: n = len - numFriends + 1.
    let n = len - num_friends + 1;
    let bytes = word.as_bytes();

    // To mimic C's strncmp behavior which reads n bytes even past the end of the
    // "logical" string (using the null terminator at word[len]), we create an extended
    // vector that contains the bytes of the string followed by n zeros.
    let mut extended = Vec::with_capacity(len + n);
    extended.extend_from_slice(bytes);
    // Append n zeros (which simulate the '\0' in C).
    extended.extend(std::iter::repeat(0).take(n));

    // ans will hold the best starting index.
    let mut ans = 0;
    // Loop through each possible starting index i (0 <= i < len)
    for i in 0..len {
        // Compare the substring starting at i with the one at ans, over n bytes.
        // The comparison is lexicographical.
        if extended[i..i + n] > extended[ans..ans + n] {
            ans = i;
        }
    }

    // Mimic the C behavior:
    // If ans + n is less than len, then the substring is truncated to length n.
    // Otherwise, the substring from ans to the end is returned.
    if ans + n < len {
        // SAFETY: Since our input is assumed to be ASCII, slicing at byte offsets is valid UTF-8.
        &word[ans..ans + n]
    } else {
        &word[ans..]
    }
}

fn main() -> io::Result<()> {
    // Use a buffered reader for input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the entire input into a string.
    reader.read_to_string(&mut input)?;

    // Split the input on whitespace.
    let mut tokens = input.split_whitespace();

    // The first token is assumed to be the word.
    let word = match tokens.next() {
        Some(token) => token,
        None => {
            writeln!(io::stderr(), "Error: No word provided in input.")?;
            return Ok(());
        }
    };

    // The second token is assumed to be numFriends, parsed as an integer.
    let num_friends_str = match tokens.next() {
        Some(token) => token,
        None => {
            writeln!(io::stderr(), "Error: No numFriends provided in input.")?;
            return Ok(());
        }
    };

    let num_friends: usize = match num_friends_str.parse() {
        Ok(num) => num,
        Err(_) => {
            writeln!(io::stderr(), "Error: numFriends is not a valid number.")?;
            return Ok(());
        }
    };

    // Call the answer_string function to get the resulting substring.
    let result = answer_string(word, num_friends);

    // Print the result with a newline, matching the original C code output format.
    println!("{}", result);

    Ok(())
}