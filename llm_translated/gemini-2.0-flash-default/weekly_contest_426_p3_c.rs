use std::io;
use std::io::Read;

fn linepots(k: i32, pots: &Vec<Vec<usize>>, node: usize, length: &Vec<usize>, visited: i32) -> i32 {
    if k == -1 {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    let mut answer = 1;
    for i in 0..length[node] {
        if pots[node][i] as i32 != visited {
            answer += linepots(k - 1, pots, pots[node][i], length, node as i32);
        }
    }
    answer
}

fn max_target_nodes(
    edges1: &Vec<Vec<i32>>,
    edges2: &Vec<Vec<i32>>,
    k: i32,
) -> Vec<i32> {
    let mut len1 = 0;
    let mut len2 = 0;

    for edge in edges1 {
        len1 = len1.max(edge[1]);
    }
    for edge in edges2 {
        len2 = len2.max(edge[1]);
    }

    let mut pots1: Vec<Vec<usize>> = Vec::new();
    let mut pots2: Vec<Vec<usize>> = Vec::new();
    let mut answer: Vec<i32> = vec![0; (len1 + 1) as usize];
    let mut length1: Vec<usize> = vec![0; (len1 + 1) as usize];
    let mut length2: Vec<usize> = vec![0; (len2 + 1) as usize];

    for i in 0..=len1 {
        let mut add = 0;
        let mut ccc: Vec<usize> = Vec::new();
        for edge in edges1 {
            if edge[0] == i {
                ccc.push(edge[1] as usize);
                add += 1;
            }
            if edge[1] == i {
                ccc.push(edge[0] as usize);
                add += 1;
            }
        }
        pots1.push(ccc);
        length1[i as usize] = pots1[i as usize].len();
    }

    for i in 0..=len2 {
        let mut add = 0;
        let mut ccc: Vec<usize> = Vec::new();
        for edge in edges2 {
            if edge[0] == i {
                ccc.push(edge[1] as usize);
                add += 1;
            }
            if edge[1] == i {
                ccc.push(edge[0] as usize);
                add += 1;
            }
        }
        pots2.push(ccc);
        length2[i as usize] = pots2[i as usize].len();
    }

    let mut max = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i as usize, &length2, -1);
        max = max.max(temp);
    }

    for i in 0..=len1 {
        answer[i as usize] = linepots(k, &pots1, i as usize, &length1, -1) + max;
    }

    answer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().parse()?;
    let mut edges1: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n1 {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        edges1.push(vec![nums.next().unwrap(), nums.next().unwrap()]);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().parse()?;
    let mut edges2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n2 {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        edges2.push(vec![nums.next().unwrap(), nums.next().unwrap()]);
    }

    // Input for k
    let k: i32 = lines.next().unwrap().parse()?;

    let result = max_target_nodes(&edges1, &edges2, k);

    for &val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}