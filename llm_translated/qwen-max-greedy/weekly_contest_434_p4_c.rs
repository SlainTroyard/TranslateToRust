use std::io::{self, BufRead, Write};

// 用于计算二进制表示中1的个数
fn popcount(n: usize) -> usize {
    n.count_ones() as usize
}

// 判断有向图是否有环
fn has_cycle(sub: usize, g: &[[usize; 26]; 26], g_size: &[usize; 26]) -> bool {
    let mut color = [0; 26]; // 0: 未访问, 1: 正在访问, 2: 已访问完成
    
    // 使用非递归DFS检测环
    for start in 0..26 {
        // 如果节点不在子集中或已被完全访问，跳过
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }
        
        // 使用栈代替递归
        let mut stack = [0; 26];
        let mut stack_pos = [0; 26]; // 记录每个节点在其邻居列表中的当前位置
        let mut top = 0;
        
        stack[top] = start;
        stack_pos[top] = 0;
        color[start] = 1; // 标记为正在访问
        
        while top >= 0 {
            let x = stack[top];
            
            // 如果已经处理完所有邻居
            if stack_pos[top] >= g_size[x] {
                color[x] = 2; // 标记为已访问完成
                top -= 1;
                continue;
            }
            
            let y = g[x][stack_pos[top]];
            stack_pos[top] += 1;
            
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
                stack[top] = y;
                stack_pos[top] = 0;
            }
        }
    }
    
    false
}

// 返回结果数组
fn supersequences(words: &Vec<String>, words_size: usize) -> Result<(Vec<Vec<usize>>, Vec<usize>), String> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = [[0; 26]; 26]; // 邻接表
    let mut g_size = [0; 26]; // 每个节点的邻居数量
    
    // 构建图和计算掩码
    for i in 0..words_size {
        let x = words[i].chars().nth(0).unwrap() as usize - 'a' as usize;
        let y = words[i].chars().nth(1).unwrap() as usize - 'a' as usize;
        
        all |= (1 << x) | (1 << y);
        
        if x == y {
            mask2 |= (1 << x);
        }
        
        g[x][g_size[x]] = y;
        g_size[x] += 1;
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
        if sub == 0 { break; }
        sub = (sub - 1) & mask1;
    }
    
    // 分配返回结果的内存
    let mut result = Vec::with_capacity(valid_subsets.len());
    let mut return_column_sizes = Vec::with_capacity(valid_subsets.len());
    
    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        
        // 计算每个字母的出现次数
        for j in 0..26 {
            row[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
        }
        
        result.push(row);
        return_column_sizes.push(26);
    }
    
    Ok((result, return_column_sizes))
}

fn main() -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    // 读取输入
    let mut input = String::new();
    stdin_lock.read_line(&mut input)?;
    let n: usize = input.trim().parse().map_err(|_| "Error reading input for n".to_string())?;
    
    // 分配内存并读取字符串数组
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input)?;
        let word = input.trim().to_string();
        if word.len() != 2 {
            return Err("Error reading input for words".to_string());
        }
        words.push(word);
    }
    
    // 调用函数计算结果
    let (result, return_column_sizes) = supersequences(&words, n)?;
    
    // 输出结果
    for (i, row) in result.iter().enumerate() {
        for &val in row {
            write!(stdout_lock, "{} ", val)?;
        }
        writeln!(stdout_lock)?;
    }
    
    Ok(())
}