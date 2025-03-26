use std::io;

struct Solution;

impl Solution {
    fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;

        for &x in &nums {
            // Update f2 using previous max_f1 and f2
            let current_max = std::cmp::max(f2, max_f1);
            f2 = current_max + if x == k { 1 } else { 0 };

            // Update f1[x] using previous f0
            let current_f1_val = std::cmp::max(f1[x as usize], f0);
            f1[x as usize] = current_f1_val + 1;

            // Update f0 if current element is k
            if x == k {
                f0 += 1;
            }

            // Update max_f1 with the new value of f1[x]
            if f1[x as usize] > max_f1 {
                max_f1 = f1[x as usize];
            }
        }

        std::cmp::max(max_f1, f2)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Parse n and k
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    // Parse the nums vector
    let nums: Vec<i32> = tokens.map(|s| s.parse().unwrap()).collect();

    // Ensure the input has exactly n numbers (problem guarantees valid input)
    assert_eq!(nums.len(), n);

    // Compute and print the result
    let solution = Solution;
    println!("{}", solution.max_frequency(nums, k));
}