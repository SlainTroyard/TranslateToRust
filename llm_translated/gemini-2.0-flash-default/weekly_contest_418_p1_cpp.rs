fn main() {
    let mut num_size = String::new();
    std::io::stdin().read_line(&mut num_size).unwrap();
    let num_size: usize = num_size.trim().parse().unwrap();

    let mut nums_str = String::new();
    std::io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let s = Solution {};
    println!("{}", s.max_good_number(nums));
}

struct Solution {}

impl Solution {
    fn max_good_number(&self, mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|&a, &b| {
            let len_a = (a as f64).log2() as i32 + 1;
            let len_b = (b as f64).log2() as i32 + 1;
            let val_a = (a as i64) << len_b | (b as i64);
            let val_b = (b as i64) << len_a | (a as i64);
            val_a.cmp(&val_b).reverse() //Reverse for descending order
        });

        let mut ans: i32 = 0;
        for &x in &nums {
            let len_x = (x as f64).log2() as i32 + 1;
            ans = (ans << len_x) | x;
        }
        ans
    }
}