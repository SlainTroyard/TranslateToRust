use std::io;

struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let mut xor_all = n ^ (n + 1);
        for (i, &num) in nums.iter().enumerate() {
            xor_all ^= (i as i32) ^ num;
        }
        let shift = (xor_all as u32).trailing_zeros();

        let mut ans = [0i32; 2];
        for (i, &num) in nums.iter().enumerate() {
            if i < (n as usize) {
                let bit = ((i as i32) >> shift) & 1;
                ans[bit as usize] ^= i as i32;
            }
            let bit = (num as u32 >> shift) & 1;
            ans[bit as usize] ^= num;
        }
        ans.to_vec()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let initial_num_size = tokens.next().unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for num in tokens {
        nums.push(num);
    }
    let solution = Solution;
    let result = solution.get_sneaky_numbers(&nums);
    println!("{} {}", result[0], result[1]);
}