fn has_cycle(sub: u32, g: &Vec<Vec<usize>>) -> bool {
    let mut color = vec![0u8; 26];
    for start in 0..26 {
        if (sub & (1 << start)) == 0 || color[start] == 2 {
            continue;
        }
        let mut stack = vec![start];
        let mut stack_pos = vec![0];
        color[start] = 1;
        while let Some(top) = stack.last() {
            let x = *top;
            let pos = *stack_pos.last().unwrap();
            if pos >= g[x].len() {
                color[x] = 2;
                stack.pop();
                stack_pos.pop();
            } else {
                let y = g[x][pos];
                let current_pos = stack_pos.last_mut().unwrap();
                *current_pos += 1;
                if (sub & (1 << y)) == 0 {
                    continue;
                }
                if color[y] == 1 {
                    return true;
                }
                if color[y] == 0 {
                    stack.push(y);
                    stack_pos.push(0);
                    color[y] = 1;
                }
            }
        }
    }
    false
}

pub fn supersequences(words: &[&str]) -> Vec<Vec<i32>> {
    let mut g: Vec<Vec<usize>> = vec![vec![]; 26];
    let mut all = 0u32;
    let mut mask2 = 0u32;
    for word in words {
        let bytes = word.as_bytes();
        let x = (bytes[0] - b'a') as usize;
        let y = (bytes[1] - b'a') as usize;
        g[x].push(y);
        all |= 1 << x;
        all |= 1 << y;
        if x == y {
            mask2 |= 1 << x;
        }
    }
    let mask1 = all ^ mask2;
    let mut valid_subsets = Vec::new();
    let mut max_size = 0;
    let mut sub = mask1;
    loop {
        let size = sub.count_ones();
        if size >= max_size && !has_cycle(sub, &g) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }
            valid_subsets.push(sub);
        }
        let new_sub = (sub - 1) & mask1;
        if new_sub == mask1 {
            break;
        }
        sub = new_sub;
    }
    let mut result = Vec::new();
    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        for j in 0..26 {
            let a = (all >> j) & 1;
            let b = ((all ^ sub) >> j) & 1;
            row[j] = (a as i32) + (b as i32);
        }
        result.push(row);
    }
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let words: Vec<&str> = lines.take(n).collect();
    if words.len() != n {
        eprintln!("Error: Not enough words provided");
        return;
    }
    let result = supersequences(&words);
    for row in &result {
        for &num in row {
            print!("{} ", num);
        }
        println!();
    }
}