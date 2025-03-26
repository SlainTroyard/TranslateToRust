use std::io::{self, BufRead};

// 计算两个数的最大公因数
fn gcd(a: isize, b: isize) -> isize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// 计算两个数的最小公倍数
fn lcm(a: isize, b: isize) -> isize {
    (a / gcd(a, b)) * b
}

// 返回最长乘积等价子数组的长度
fn max_length(nums: &[isize]) -> isize {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod = 1;
        let mut g = nums[i];
        let mut l = nums[i];
        for j in i..nums.len() {
            if prod > isize::MAX / nums[j] {
                break; // 防止溢出
            }
            prod *= nums[j];
            g = gcd(g, nums[j]);
            l = lcm(l, nums[j]);
            if prod == l * g {
                let length = (j - i + 1) as isize;
                if length > max_length {
                    max_length = length;
                }
            }
        }
    }
    max_length
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取输入
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");
    let mut nums: Vec<isize> = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: isize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");
        nums.push(num);
    }

    // 计算结果
    let result = max_length(&nums);

    // 输出结果
    println!("{}", result);
}