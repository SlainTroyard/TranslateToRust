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
            right += 1;
        } else {
            right += 1;
        }
    }

    res
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse input
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let k: i32 = lines.next().unwrap().trim().parse().unwrap();

    // Calculate result
    let result = number_of_substrings(s, k);

    // Print result
    println!("{}", result);

    Ok(())
}