use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    fn get_largest_outlier(nums: &[i32]) -> i32 {
        let mut ctr: HashMap<i32, i32> = HashMap::new();
        let mut sm = 0;

        // Count occurrences and calculate sum
        for &num in nums {
            *ctr.entry(num).or_insert(0) += 1;
            sm += num;
        }

        // Extract unique numbers and sort in descending order
        let mut cands: Vec<i32> = ctr.keys().cloned().collect();
        cands.sort_unstable_by(|a, b| b.cmp(a));

        // Check each candidate
        for &n in &cands {
            let d = (sm - n) / 2;
            let m = (sm - n) % 2;
            // Check if d exists in the map and satisfies the conditions
            if m == 0 && ctr.contains_key(&d) && (d != n || ctr[&d] > 1) {
                return n;
            }
        }
        -1
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Input the number of elements
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Invalid input");

    // Input the elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        nums.push(input.trim().parse::<i32>().expect("Invalid input"));
    }

    // Call the method and output the result
    let result = Solution::get_largest_outlier(&nums);
    println!("{}", result);

    Ok(())
}