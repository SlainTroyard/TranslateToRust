use std::io;

/// 计算两个数的最大公因数，返回i64类型
fn gcd(a: i32, b: i32) -> i64 {
    let mut a = a as i64;
    let mut b = b as i64;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// 计算两个数的最小公倍数，返回i64类型
fn lcm(a: i32, b: i32) -> i64 {
    let g = gcd(a, b);
    (a as i64 / g) * (b as i64)
}

/// 返回最长乘积等价子数组的长度
fn max_length(nums: &[i32]) -> i32 {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod: i64 = 1;
        let mut g: i64 = nums[i] as i64;
        let mut l: i64 = nums[i] as i64;
        for j in i..nums.len() {
            // 检查乘法溢出
            if let Some(new_prod) = prod.checked_mul(nums[j] as i64) {
                prod = new_prod;
            } else {
                break;
            }

            // 更新GCD和LCM
            g = gcd(g as i32, nums[j]) as i64;
            l = lcm(l as i32, nums[j]);

            // 检查条件
            if prod == g * l {
                let current_length = j - i + 1;
                if current_length > max_length as usize {
                    max_length = current_length as i32;
                }
            }
        }
    }
    max_length
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let num_size: usize = tokens.next()
        .expect("Invalid input: missing number size")
        .parse()
        .expect("Invalid input: number size must be integer");
    let nums: Vec<i32> = tokens
        .map(|s| s.parse().expect("Invalid input: elements must be integers"))
        .take(num_size)
        .collect();

    if nums.len() != num_size {
        panic!("Input does not contain enough elements");
    }

    let result = max_length(&nums);
    println!("{}", result);
}