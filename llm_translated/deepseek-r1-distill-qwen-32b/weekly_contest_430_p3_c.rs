use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn read_i32() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_array(n: i32) -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(parts.len(), n as usize);
    parts
}

fn number_of_subsequences(nums: &[i32]) -> u64 {
    let mut map = HashMap::new();
    let n = nums.len();
    let mut ans = 0;

    for i in 4..(n - 2) {
        let c = nums[i];
        for j in (i + 2)..n {
            let d = nums[j];
            let g = gcd(c, d);
            if g == 0 {
                continue;
            }
            let d_div_g = d / g;
            let c_div_g = c / g;
            let key = (d_div_g, c_div_g);
            map.entry(key).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    for i in 2..(n - 4) {
        let b = nums[i];
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            if g == 0 {
                continue;
            }
            let a_div_g = a / g;
            let b_div_g = b / g;
            let key = (a_div_g, b_div_g);
            ans += map.get(&key).unwrap_or(&0);
        }

        let c = nums[i + 2];
        for j in (i + 4)..n {
            let d = nums[j];
            let g = gcd(c, d);
            if g == 0 {
                continue;
            }
            let d_div_g = d / g;
            let c_div_g = c / g;
            let key = (d_div_g, c_div_g);
            map.entry(key).and_modify(|v| *v -= 1).or_insert(-1);
        }
    }

    ans as u64
}

fn main() {
    let n = read_i32();
    let nums = read_array(n);
    let result = number_of_subsequences(&nums);
    println!("{}", result);
}