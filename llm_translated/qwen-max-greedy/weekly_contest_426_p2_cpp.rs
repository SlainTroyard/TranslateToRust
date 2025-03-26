use std::collections::HashMap;
use std::io;

fn get_largest_outlier(nums: &Vec<i32>) -> i32 {
    let mut ctr: HashMap<i32, i32> = HashMap::new();
    let mut sm = 0;

    for &num in nums.iter() {
        *ctr.entry(num).or_insert(0) += 1;
        sm += num;
    }

    let mut cands: Vec<i32> = ctr.keys().cloned().collect();
    cands.sort_unstable_by(|a, b| b.cmp(a));

    for &n in cands.iter() {
        let d = (sm - n) / 2;
        let m = (sm - n) % 2;
        if m == 0 && ctr.contains_key(&d) && (d != n || *ctr.get(&d).unwrap() > 1) {
            return n;
        }
    }
    -1
}

fn main() {
    let mut solution = Solution {};

    // Input the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Input the elements of the array
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Call the method and output the result
    let result = solution.get_largest_outlier(&nums);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    fn get_largest_outlier(&self, nums: &Vec<i32>) -> i32 {
        get_largest_outlier(nums)
    }
}