use std::io::{self, Read};

fn number_of_substrings(s: &str, k: i32) -> i32 {
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s_l = s.len();
    let mut res = 0;

    while left < s_l && right < s_l {
        hash_arr[(s.as_bytes()[right] - b'a') as usize] += 1;
        if hash_arr[(s.as_bytes()[right] - b'a') as usize] == k as usize {
            res += (s_l - right) as i32;
            while left <= right {
                hash_arr[(s.as_bytes()[left] - b'a') as usize] -= 1;
                left += 1;
                if hash_arr[(s.as_bytes()[left - 1] - b'a') as usize] != (k - 1) as usize {
                    res += (s_l - right) as i32;
                } else {
                    break;
                }
            }
        }
        right += 1;
    }

    res
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input into s and k
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().unwrap_or("");
    let k: i32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

    let result = number_of_substrings(s, k);
    println!("{}", result);

    Ok(())
}