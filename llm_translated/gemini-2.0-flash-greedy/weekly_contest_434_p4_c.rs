use std::io;
use std::io::Read;
use std::vec::Vec;

// 用于计算二进制表示中1的个数
fn popcount(n: i32) -> i32 {
    let mut count = 0;
    let mut n_mut = n;
    while n_mut != 0 {
        count += n_mut & 1;
        n_mut >>= 1;
    }
    count
}

// 判断有向图是否有环
fn has_cycle(sub: i32, g: &[[i32; 26]; 26], g_size: &[i32; 26]) -> bool {
    let mut color = [0; 26]; // 0: 未访问, 1: 正在访问, 2: 已访问完成

    // 使用非递归DFS检测环
    for start in 0..26 {
        // 如果节点不在子集中或已被完全访问，跳过
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }

        // 使用栈代替递归
        let mut stack: [usize; 26] = [0; 26];
        let mut stack_pos: [usize; 26] = [0; 26]; // 记录每个节点在其邻居列表中的当前位置
        let mut top: i32 = -1;

        top += 1;
        stack[top as usize] = start;
        stack_pos[top as usize] = 0;
        color[start] = 1; // 标记为正在访问

        while top >= 0 {
            let x = stack[top as usize];

            // 如果已经处理完所有邻居
            if stack_pos[top as usize] >= g_size[x] as usize {
                color[x] = 2; // 标记为已访问完成
                top -= 1;
                continue;
            }

            let y = g[x][stack_pos[top as usize] as usize];
            stack_pos[top as usize] += 1;

            // 如果y不在当前子集中，跳过
            if (sub >> y & 1) == 0 {
                continue;
            }

            // 如果y正在被访问，说明有环
            if color[y] == 1 {
                return true;
            }

            // 如果y未被访问，将y入栈
            if color[y] == 0 {
                color[y] = 1; // 标记为正在访问
                top += 1;
                stack[top as usize] = y;
                stack_pos[top as usize] = 0;
            }
        }
    }

    false
}

fn supersequences(words: &Vec<String>) -> Option<(Vec<Vec<i32>>, Vec<i32>)> {
    let words_size = words.len();
    let mut all = 0;
    let mut mask2 = 0;
    let mut g: [[i32; 26]; 26] = [[0; 26]; 26]; // 邻接表
    let mut g_size: [i32; 26] = [0; 26]; // 每个节点的邻居数量

    // 构建图和计算掩码
    for i in 0..words_size {
        let word = &words[i];
        let x = (word.as_bytes()[0] - b'a') as usize;
        let y = (word.as_bytes()[1] - b'a') as usize;

        all |= 1 << x;
        all |= 1 << y;

        if x == y {
            mask2 |= 1 << x;
        }

        g[x][g_size[x] as usize] = y as i32;
        g_size[x] += 1;
    }

    // 计算mask1
    let mask1 = all ^ mask2;

    // 使用动态增长的数组
    let mut valid_subsets: Vec<i32> = Vec::new();
    let mut max_size = 0;

    // 枚举mask1的所有子集
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

        // 计算mask1的下一个子集
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
    }

    // 构建结果
    let valid_subsets_count = valid_subsets.len();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut return_column_sizes: Vec<i32> = Vec::new();

    if valid_subsets_count > 0 {
        for i in 0..valid_subsets_count {
            let sub = valid_subsets[i];
            let mut row: Vec<i32> = vec![0; 26];
            return_column_sizes.push(26);

            // 计算每个字母的出现次数
            for j in 0..26 {
                row[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
            }
            result.push(row);
        }
        Some((result, return_column_sizes))
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut words: Vec<String> = Vec::new();
    for _ in 0..n {
        words.push(lines.next().unwrap().to_string());
    }

    match supersequences(&words) {
        Some((result, return_column_sizes)) => {
            for i in 0..result.len() {
                for j in 0..return_column_sizes[i] {
                    print!("{} ", result[i][j as usize]);
                }
                println!();
            }
        }
        None => {}
    }

    Ok(())
}