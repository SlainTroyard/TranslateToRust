use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn get_largest_outlier(nums: &Vec<i32>) -> i32 {
        let mut ctr: HashMap<i32, i32> = HashMap::new();
        let mut sm: i32 = 0;

        for &num in nums {
            *ctr.entry(num).or_insert(0) += 1;
            sm += num;
        }

        let mut cands: Vec<i32> = ctr.keys().cloned().collect();
        cands.sort_by(|a, b| b.cmp(a));

        for &n in &cands {
            let d = (sm - n) / 2;
            let m = (sm - n) % 2;
            if m == 0 && ctr.contains_key(&d) && (d != n || *ctr.get(&d).unwrap() > 1) {
                return n;
            }
        }
        -1
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input the elements of the array
    let nums: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Call the method and output the result
    let result = Solution::get_largest_outlier(&nums);
    println!("{}", result);

    Ok(())
}