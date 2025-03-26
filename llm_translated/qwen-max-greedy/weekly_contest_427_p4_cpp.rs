use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::io::{self, BufRead, Write};

// tweak for performance
const BLOCK_SIZE: usize = 512;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Query {
    lx: usize,
    rx: usize,
    ly: usize,
    ry: usize,
    area: isize,
}

impl Query {
    fn new(lx: usize, rx: usize, ly: usize, ry: usize, area: isize) -> Self {
        Query { lx, rx, ly, ry, area }
    }
}

struct Solution;

impl Solution {
    fn max_rectangle_area(x_coord: &mut Vec<isize>, y_coord: &mut Vec<isize>) -> isize {
        let mut ans = -1;
        let n = x_coord.len();

        // Coordinate compression
        let mut to_compressed: HashMap<isize, usize> = HashMap::new();
        let mut to_original: HashMap<usize, isize> = HashMap::new();
        let mut p: BTreeSet<isize> = BTreeSet::new();
        p.extend(x_coord.iter().cloned());
        p.extend(y_coord.iter().cloned());

        let mut lst = 0;
        for &a in &p {
            to_compressed.insert(a, lst);
            to_original.insert(lst, a);
            lst += 1;
        }

        for x in x_coord.iter_mut() {
            *x = *to_compressed.get(x).unwrap();
        }
        for y in y_coord.iter_mut() {
            *y = *to_compressed.get(y).unwrap();
        }

        let mut by_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut by_y: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 0..n {
            by_x.entry(x_coord[i] as usize).or_insert_with(Vec::new).push(y_coord[i] as usize);
            by_y.entry(y_coord[i] as usize).or_insert_with(Vec::new).push(x_coord[i] as usize);
        }

        for (_, v) in by_x.iter_mut() {
            v.sort_unstable();
        }
        for (_, v) in by_y.iter_mut() {
            v.sort_unstable();
        }

        let mut queries: Vec<Query> = Vec::new();
        for i in 0..n {
            let ax = x_coord[i] as usize;
            let ay = y_coord[i] as usize;

            if let Some(rx) = by_y[&ay].iter().find(|&&x| x > ax) {
                let rx = *rx;
                let ry = ay;

                if let Some(ty) = by_x[&ax].iter().find(|&&y| y > ay) {
                    let ty = *ty;
                    let tx = ax;

                    if by_x[&rx].iter().any(|&y| y == *ty) && by_y[&ty].iter().any(|&x| x == *rx) {
                        let dx = ax + 1;
                        let dy = ay + 1;
                        let ux = rx - 1;
                        let uy = ty - 1;
                        let area = (to_original[&(*ty as isize)] - to_original[&(*ay as isize)]) as isize
                            * (to_original[&(*rx as isize)] - to_original[&(*ax as isize)]) as isize;

                        if dx <= ux && dy <= uy {
                            queries.push(Query::new(dx, ux, dy, uy, area));
                        } else {
                            ans = ans.max(area);
                        }
                    }
                }
            }
        }

        queries.sort_by_key(|q| (q.lx / BLOCK_SIZE, q.rx));

        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut ms: HashSet<usize> = HashSet::new();

        for q in queries {
            while cur_l > q.lx {
                cur_l -= 1;
                ms.extend(by_x[&cur_l].iter().cloned());
            }
            while cur_r < q.rx {
                cur_r += 1;
                ms.extend(by_x[&cur_r].iter().cloned());
            }
            while cur_l < q.lx {
                for &b in &by_x[&cur_l] {
                    ms.remove(&b);
                }
                cur_l += 1;
            }
            while cur_r > q.rx {
                for &b in &by_x[&cur_r] {
                    ms.remove(&b);
                }
                cur_r -= 1;
            }
            if let Some(&it) = ms.iter().find(|&&y| y >= q.ly) {
                if it <= q.ry {
                    continue; // there is a point that lies inside
                }
            }
            ans = ans.max(q.area);
        }

        ans
    }
}

fn main() {
    let mut solution = Solution;

    // Input the number of points
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Failed to parse number of points");

    let mut x_coord: Vec<isize> = vec![0; n];
    let mut y_coord: Vec<isize> = vec![0; n];

    // Input the coordinates of the points
    for i in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        let coords: Vec<isize> = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coordinate"))
            .collect();
        x_coord[i] = coords[0];
        y_coord[i] = coords[1];
    }

    // Calculate the maximum rectangle area
    let max_area = solution.max_rectangle_area(&mut x_coord, &mut y_coord);

    // Output the result
    println!("{}", max_area);
}