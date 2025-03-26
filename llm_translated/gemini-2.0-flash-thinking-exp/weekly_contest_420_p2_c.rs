fn number_of_substrings(s: &str, k: i32) -> i32 {
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s_l = s.len();
    let mut res = 0;

    while left < s_l && right < s_l {
        let char_index = (s.as_bytes()[right] - b'a') as usize;
        hash_arr[char_index] += 1;

        if hash_arr[char_index] == k {
            res += (s_l - right) as i32;
            while left <= right {
                let left_char_index = (s.as_bytes()[left] - b'a') as usize;
                hash_arr[left_char_index] -= 1;
                left += 1;
                if hash_arr[left_char_index] != k - 1 {
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

fn main() {
    use std::io;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let parts: Vec<&str> = input_line.trim().split_whitespace().collect();

    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid integer input");

    let result = number_of_substrings(s, k);
    println!("{}", result);
}