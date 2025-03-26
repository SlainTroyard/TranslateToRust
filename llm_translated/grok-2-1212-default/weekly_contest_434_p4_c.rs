use std::io::{self, BufRead};

fn popcount(n: i32) -> i32 {
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

fn has_cycle(sub: i32, g: &[[i32; 26]; 26], g_size: &[i32; 26]) -> bool {
    let mut color = [0; 26]; // 0: 未访问, 1: 正在访问, 2: 已访问完成

    for start in 0..26 {
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = vec![start];
        let mut stack_pos = vec![0];
        color[start] = 1; // 标记为正在访问

        while let Some(&x) = stack.last() {
            if stack_pos.last().unwrap() >= &g_size[x] {
                color[x] = 2; // 标记为已访问完成
                stack.pop();
                stack_pos.pop();
                continue;
            }

            let y = g[x][*stack_pos.last().unwrap() as usize];
            *stack_pos.last_mut().unwrap() += 1;

            if (sub >> y & 1) == 0 {
                continue;
            }

            if color[y as usize] == 1 {
                return true;
            }

            if color[y as usize] == 0 {
                color[y as usize] = 1; // 标记为正在访问
                stack.push(y as usize);
                stack_pos.push(0);
            }
        }
    }

    false
}

fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = [[0; 26]; 26];
    let mut g_size = [0; 26];

    for word in words {
        let x = word.chars().nth(0).unwrap() as i32 - 'a' as i32;
        let y = word.chars().nth(1).unwrap() as i32 - 'a' as i32;

        all |= 1 << x;
        all |= 1 << y;

        if x == y {
            mask2 |= 1 << x;
        }

        g[x as usize][g_size[x as usize] as usize] = y;
        g_size[x as usize] += 1;
    }

    let mask1 = all ^ mask2;

    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    let mut sub = mask1;
    loop {
        let size = popcount(sub);

        if size >= max_size && !has_cycle(sub, &g, &g_size) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }

            valid_subsets.push(sub);
        }

        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
    }

    let mut result = Vec::new();

    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        for j in 0..26 {
            row[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
        }
        result.push(row);
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取输入
    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // 读取字符串数组
    let mut words = Vec::new();
    for _ in 0..n {
        let word = lines.next().unwrap()?.trim().to_string();
        words.push(word);
    }

    // 计算结果
    let result = supersequences(&words);

    // 输出结果
    for row in result {
        for &val in &row {
            print!("{} ", val);
        }
        println!();
    }

    Ok(())
}