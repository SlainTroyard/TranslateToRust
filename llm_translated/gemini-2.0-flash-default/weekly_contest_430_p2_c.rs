use std::io;
use std::cmp::Ordering;

fn answer_string(word: &mut String, num_friends: i32) -> &str {
    if num_friends == 1 {
        return word;
    }

    let len = word.len();
    let n = len - num_friends as usize + 1;
    let mut ans = 0;

    for i in 0..len - n + 1 {
        match word[i..i + n].cmp(&word[ans..ans + n]) {
            Ordering::Greater => {
                ans = i;
            }
            _ => {}
        }
    }

    if ans + n < len {
        word.truncate(ans + n);
    }

    &word[ans..]
}

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim().to_string();

    let mut num_friends_str = String::new();
    io::stdin().read_line(&mut num_friends_str).expect("Failed to read line");
    let num_friends: i32 = num_friends_str.trim().parse().expect("Invalid input");

    let mut word_mut = word.clone();
    let result = answer_string(&mut word_mut, num_friends);

    println!("{}", result);
}