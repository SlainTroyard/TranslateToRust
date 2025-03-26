use std::collections::HashMap;
use std::io;

struct Solution {}

impl Solution {
    fn get_largest_outlier(nums: &Vec<i32>) -> i32 {
        let mut ctr: HashMap<i32, i32> = HashMap::new();
        let mut sm: i32 = 0;

        for &num in nums {
            *ctr.entry(num).or_insert(0) += 1;
            sm += num;
        }

        let mut cands: Vec<i32> = ctr.keys().cloned().collect();
        cands.sort_by(|a, b| b.cmp(a)); // Sort in descending order

        for &n in &cands {
            let d = (sm - n) / 2;
            let m = (sm - n) % 2;
            if m == 0 {
                if let Some(&count) = ctr.get(&d) {
                    if d != n || count > 1 {
                        return n;
                    }
                }
            }
        }
        return -1;
    }
}

fn main() {
    let mut input = String::new();

    // Read the number of elements
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    // Read the elements of the array
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let solution = Solution {};
    let result = solution.get_largest_outlier(&nums);
    println!("{}", result);
}