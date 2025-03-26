use std::cmp::{max, min};
use std::io;
use std::io::Read;
use std::str::FromStr;

fn max_difference(s: &str, k: i32) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = [0; 5]; // 当前窗口中各数字的出现次数
            let mut pre_s = [0; 5]; // 窗口左边部分的数字出现次数
            let mut min_s = [[INF, INF], [INF, INF]]; // 最小的差值
            let mut left = 0;

            for (i, c) in s.chars().enumerate() {
                let digit = (c as u8 - b'0') as usize;
                cur_s[digit] += 1; // 更新当前窗口中数字的出现次数
                let r = (i + 1) as i32;

                // 当窗口大小 >= k 且满足其他条件时，尝试缩小窗口左边界
                while r - (left as i32) >= k
                    && cur_s[x] > pre_s[x]
                    && cur_s[y] > pre_s[y]
                {
                    let p = &mut min_s[pre_s[x] % 2][pre_s[y] % 2];
                    *p = min(*p, pre_s[x] - pre_s[y]); // 更新最小差值
                    let left_digit = (s.chars().nth(left).unwrap() as u8 - b'0') as usize;
                    pre_s[left_digit] += 1; // 更新窗口左边部分的数字出现次数
                    left += 1;
                }

                // 更新答案
                ans = max(
                    ans,
                    cur_s[x] - cur_s[y] - min_s[(cur_s[x] % 2) ^ 1][cur_s[y] % 2],
                );
            }
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut iter = input.split_whitespace();
    let s = iter.next().ok_or("Missing string input")?;
    let k: i32 = iter
        .next()
        .ok_or("Missing integer input")?
        .parse()
        .map_err(|_| "Invalid integer input")?;

    // 调用函数计算结果
    let result = max_difference(s, k);

    // 输出结果
    println!("{}", result);

    Ok(())
}