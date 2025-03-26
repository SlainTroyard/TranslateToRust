// Problem: Weekly Contest 420 Problem 3

const MX: usize = 1_000_001;
static mut LPF: [usize; MX] = [0; MX];

fn init() {
    unsafe {
        for i in 2..MX {
            if LPF[i] == 0 {
                for j in (i..MX).step_by(i) {
                    if LPF[j] == 0 {
                        LPF[j] = i;
                    }
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    fn min_operations(nums: &mut Vec<usize>) -> isize {
        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                unsafe {
                    nums[i] = LPF[nums[i]];
                }
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    init();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    let mut nums: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: usize = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    let sol = Solution;
    let result = sol.min_operations(&mut nums);
    println!("{}", result);
}