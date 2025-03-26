use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::mem;

// 用于计算二进制表示中1的个数
fn popcount(n: i32) -> i32 {
    n.count_ones() as i32
}

// 判断有向图是否有环
fn has_cycle(sub: i32, g: &[Vec<i32>], g_size: &[i32]) -> bool {
    let mut color = [0; 26]; // 0: 未访问, 1: 正在访问, 2: 已访问完成
    
    // 使用非递归DFS检测环
    for start in 0..26 {
        // 如果节点不在子集中或已被完全访问，跳过
        if ((sub >> start) & 1) == 0 || color[start] == 2 {
            continue;
        }
        
        // 使用栈代替递归
        let mut stack = VecDeque::new();
        let mut stack_pos = VecDeque::new();
        
        stack.push_back(start);
        stack_pos.push_back(0);
        color[start] = 1; // 标记为正在访问
        
        while let Some(x) = stack.pop_back() {
            let pos = stack_pos.pop_back().unwrap();
            
            // 如果已经处理完所有邻居
            if pos >= g_size[x] as usize {
                color[x] = 2; // 标记为已访问完成
                continue;
            }
            
            let y = g[x][pos] as usize;
            stack_pos.push_back(pos + 1);
            
            // 如果y不在当前子集中，跳过
            if ((sub >> y) & 1) == 0 {
                continue;
            }
            
            // 如果y正在被访问，说明有环
            if color[y] == 1 {
                return true;
            }
            
            // 如果y未被访问，将y入栈
            if color[y] == 0 {
                color[y] = 1; // 标记为正在访问
                stack.push_back(x);
                stack_pos.push_back(pos);
                stack.push_back(y);
                stack_pos.push_back(0);
            }
        }
    }
    
    false
}

fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = vec![Vec::new(); 26]; // 邻接表
    let mut g_size = [0; 26]; // 每个节点的邻居数量
    
    // 构建图和计算掩码
    for word in words {
        let x = (word.chars().nth(0).unwrap() as u8 - b'a') as i32;
        let y = (word.chars().nth(1).unwrap() as u8 - b'a') as i32;
        
        all |= (1 << x) | (1 << y);
        
        if x == y {
            mask2 |= (1 << x);
        }
        
        g[x as usize].push(y);
        g_size[x as usize] += 1;
    }
    
    // 计算mask1
    let mask1 = all ^ mask2;
    
    // 将validSubsets分配到堆上而不是栈上
    let mut valid_subsets = Vec::new();
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
    let mut result = Vec::new();
    for sub in valid_subsets {
        let mut row = Vec::with_capacity(26);
        for j in 0..26 {
            row.push(((all >> j) & 1) + (((all ^ sub) >> j) & 1));
        }
        result.push(row);
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // 读取输入
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // 分配内存并读取字符串数组
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }
    
    // 调用函数计算结果
    let result = supersequences(&words);
    
    // 输出结果
    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}