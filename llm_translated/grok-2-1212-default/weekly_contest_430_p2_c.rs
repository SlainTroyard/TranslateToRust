use std::io::{self, Read};

fn answer_string(word: &mut String, num_friends: usize) -> String {
    if num_friends == 1 {
        return word.clone();
    }

    let len = word.len();
    let n = len - num_friends + 1;
    let mut ans = 0;

    for i in 0..len {
        if word[i..].starts_with(&word[ans..n]) && word[i..i+n] > word[ans..ans+n] {
            ans = i;
        }
    }

    if ans + n < len {
        word.truncate(ans + n);
    }

    word[ans..].to_string()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Read word
    let mut word = lines.next().unwrap().to_string();

    // Read num_friends
    let num_friends: usize = lines.next().unwrap().trim().parse().unwrap();

    // Call answer_string function
    let result = answer_string(&mut word, num_friends);

    // Print the result
    println!("{}", result);

    Ok(())
}