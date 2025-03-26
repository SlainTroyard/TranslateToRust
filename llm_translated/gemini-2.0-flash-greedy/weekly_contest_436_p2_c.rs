use std::cmp::min;
use std::io;
use std::io::Read;
use std::process::exit;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = min(index[element as usize], i as i32);
    }

    let mut ans = vec![i32::MAX; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        for j in 1..=((group as f64).sqrt() as i32) {
            if group % j == 0
                && (index[j as usize] != i32::MAX || index[(group / j) as usize] != i32::MAX)
            {
                ans[i] = min(ans[i], min(index[(group / j) as usize], index[j as usize]));
            }
        }
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let n: usize = first_split.next().unwrap().parse()?;
    let m: usize = first_split.next().unwrap().parse()?;

    let mut groups = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        groups.push(line.parse::<i32>()?);
    }

    let mut elements = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap();
        elements.push(line.parse::<i32>()?);
    }

    let ans = assign_elements(&groups, &elements);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}