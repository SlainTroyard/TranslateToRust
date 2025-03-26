fn number_of_substrings(s: &str, k: i32) -> i32 {
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s_l = s.len();
    let mut res = 0;

    let s_bytes = s.as_bytes();

    while left < s_l && right < s_l {
        let idx = (s_bytes[right] - b'a') as usize;
        hash_arr[idx] += 1;

        if hash_arr[idx] == k {
            res += s_l - right;

            while left <= right {
                let left_idx = (s_bytes[left] - b'a') as usize;
                hash_arr[left_idx] -= 1;

                if hash_arr[left_idx] != k - 1 {
                    left += 1;
                    res += s_l - right;
                } else {
                    left += 1;
                    break;
                }
            }
            right += 1;
        } else {
            right += 1;
        }
    }

    res as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut iter = input.trim().split_whitespace();
    let s = iter.next().unwrap();
    let k: i32 = iter.next().unwrap().parse()?;

    println!("{}", number_of_substrings(s, k));

    Ok(())
}