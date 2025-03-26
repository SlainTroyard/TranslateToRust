use std::io;
use std::io::Read;

// 计算两个数的最大公因数
fn gcd(a: i64, b: i64) -> i64 {
    let mut a_mut = a;
    let mut b_mut = b;
    while b_mut != 0 {
        let temp = b_mut;
        b_mut = a_mut % b_mut;
        a_mut = temp;
    }
    a_mut
}

// 计算两个数的最小公倍数
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// 返回最长乘积等价子数组的长度
fn max_length(nums: &[i32]) -> i32 {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod: i64 = 1;
        let mut g: i64 = nums[i] as i64;
        let mut l: i64 = nums[i] as i64;
        for j in i..nums.len() {
            let num_j = nums[j] as i64;
            if prod > i64::MAX / num_j {
                break; // 防止溢出
            }
            prod *= num_j;
            g = gcd(g, num_j);
            l = lcm(l, num_j);

            if prod == l * g {
                let length = (j - i + 1) as i32;
                if length > max_length {
                    max_length = length;
                }
            }
        }
    }
    max_length
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let num_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = max_length(&nums);
    println!("{}", result);

    Ok(())
}