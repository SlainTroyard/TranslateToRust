use std::io;

const MX: usize = 1_000_001;

fn min_operations(nums: &mut Vec<i32>, lpf: &[i32]) -> i32 {
    let mut ans = 0;
    let n = nums.len();
    for i in (0..n - 1).rev() {
        if nums[i] > nums[i + 1] {
            let current = nums[i] as usize;
            let lpf_val = lpf[current];
            nums[i] = lpf_val;
            if nums[i] > nums[i + 1] {
                return -1;
            }
            ans += 1;
        }
    }
    ans
}

fn main() {
    let mut lpf = vec![0; MX];
    for i in 2..MX {
        if lpf[i] == 0 {
            for j in (i..MX).step_by(i) {
                if lpf[j] == 0 {
                    lpf[j] = i as i32;
                }
            }
        }
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens.next().unwrap().parse().unwrap();
        nums.push(num);
    }

    let ans = min_operations(&mut nums, &lpf);
    println!("{}", ans);
}