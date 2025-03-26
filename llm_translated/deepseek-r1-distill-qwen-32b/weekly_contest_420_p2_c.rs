use std::io;

// Function to count the number of valid substrings
fn number_of_substrings(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let s_l = bytes.len();
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;

    while left < s_l && right < s_l {
        let idx = (bytes[right] - b'a') as usize;
        hash_arr[idx] += 1;

        if hash_arr[idx] == k {
            // Add all possible substrings starting at 'right'
            res += (s_l - right) as i32;

            // Move left pointer to find the minimal window
            while left <= right {
                let left_idx = (bytes[left] - b'a') as usize;
                hash_arr[left_idx] -= 1;
                left += 1;

                if hash_arr[left_idx] != k - 1 {
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k = parts[1].parse::<i32>().expect("Invalid k");

    println!("{}", number_of_substrings(s, k));
}