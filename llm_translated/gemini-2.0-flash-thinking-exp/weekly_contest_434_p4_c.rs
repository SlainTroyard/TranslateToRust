// Problem: Weekly Contest 434 Problem 4
use std::io;
use std::vec;

// 用于计算二进制表示中1的个数
fn popcount(n: i32) -> i32 {
    n.count_ones() as i32
}

// 判断有向图是否有环
fn has_cycle(sub: i32, g: &Vec<Vec<i32>>) -> bool {
    let mut color = vec![0; 26]; // 0: 未访问, 1: 正在访问, 2: 已访问完成

    // 使用非递归DFS检测环
    for start in 0..26 {
        // 如果节点不在子集中或已被完全访问，跳过
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }

        // 使用栈代替递归
        let mut stack = vec![start];
        let mut stack_pos = vec![0; 26]; // 记录每个节点在其邻居列表中的当前位置
        let mut top = 0;

        color[start] = 1; // 标记为正在访问

        while top >= 0 {
            let x = stack[top];

            // 如果已经处理完所有邻居
            if stack_pos[x] as usize >= g[x].len() {
                color[x] = 2; // 标记为已访问完成
                stack.pop();
                top -= 1;
                continue;
            }

            let y = g[x][stack_pos[x] as usize];
            stack_pos[x] += 1;

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
                stack.push(y);
                top += 1;
                stack_pos[y] = 0;
            }
        }
    }

    false
}

fn supersequences(words: &Vec<String>) -> Vec<Vec<i32>> {
    let mut all_mask = 0;
    let mut mask2 = 0;
    let mut g: Vec<Vec<i32>> = vec![vec![]; 26]; // 邻接表

    // 构建图和计算掩码
    for word in words {
        let x = (word.as_bytes()[0] - b'a') as usize;
        let y = (word.as_bytes()[1] - b'a') as usize;

        all_mask |= (1 << x) | (1 << y);

        if x == y {
            mask2 |= (1 << x);
        }

        g[x].push(y as i32);
    }

    // 计算mask1
    let mask1 = all_mask ^ mask2;

    let mut valid_subsets: Vec<i32> = Vec::new();
    let mut max_size = 0;

    // 枚举mask1的所有子集
    let mut sub = mask1;
    loop {
        let size = popcount(sub);

        if size >= max_size && !has_cycle(sub, &g) {
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
        if sub == mask1 { // To mimic the do-while loop exit condition from C
            break;
        }
    }

    let mut result: Vec<Vec<i32>> = Vec::new();

    if !valid_subsets.is_empty() {
        for sub in &valid_subsets {
            let mut row = vec![0; 26];
            for j in 0..26 {
                row[j] = ((all_mask >> j) & 1) + (((all_mask ^ sub) >> j) & 1);
            }
            result.push(row);
        }
    }

    result
}

fn main() {
    // 读取输入
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut words: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        words.push(word.trim().to_string());
    }

    // 调用函数计算结果
    let result = supersequences(&words);

    // 输出结果
    for row in result {
        for j in 0..row.len() {
            print!("{} ", row[j]);
        }
        println!();
    }
}