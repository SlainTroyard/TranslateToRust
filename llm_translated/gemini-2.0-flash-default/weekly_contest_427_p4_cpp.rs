use std::cmp::max;
use std::collections::{HashMap, HashSet, BTreeSet, Multiset};
use std::io;
use std::io::prelude::*;

// Tweak for performance
const BLOCK_SIZE: usize = 512;

#[derive(Debug, Eq, PartialEq)]
struct Query {
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
    area: i64,
}

impl Query {
    fn new(lx: i32, rx: i32, ly: i32, ry: i32, area: i64) -> Self {
        Query {
            lx,
            rx,
            ly,
            ry,
            area,
        }
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.lx / BLOCK_SIZE, self.rx).cmp(&(other.lx / BLOCK_SIZE, other.rx))
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution {}

impl Solution {
    fn maxRectangleArea(x_coord: &mut Vec<i32>, y_coord: &mut Vec<i32>) -> i64 {
        let mut ans = -1;
        let n = x_coord.len();

        // Coordinate compression
        let mut to_compressed: HashMap<i32, i32> = HashMap::new();
        let mut to_original: HashMap<i32, i32> = HashMap::new();

        {
            let mut p: BTreeSet<i32> = BTreeSet::new();
            for &x in x_coord.iter() {
                p.insert(x);
            }
            for &y in y_coord.iter() {
                p.insert(y);
            }

            let mut lst = 0;
            for &a in p.iter() {
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
        }

        let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..n {
            by_x.entry(x_coord[i]).or_insert(Vec::new()).push(y_coord[i]);
            by_y.entry(y_coord[i]).or_insert(Vec::new()).push(x_coord[i]);
        }

        for (_, v) in by_x.iter_mut() {
            v.sort();
        }
        for (_, v) in by_y.iter_mut() {
            v.sort();
        }

        let mut queries: Vec<Query> = Vec::new();
        for i in 0..n {
            let ax = x_coord[i];
            let ay = y_coord[i];

            let by_y_ay = by_y.get(&ay).unwrap();
            let it = by_y_ay.binary_search(&ax).unwrap_err();
            if it == by_y_ay.len() {
                continue;
            }
            let rx = by_y_ay[it];
            let ry = ay; // this is bottom right point

            let by_x_ax = by_x.get(&ax).unwrap();
            let it = by_x_ax.binary_search(&ay).unwrap_err();
            if it == by_x_ax.len() {
                continue;
            }
            let tx = ax;
            let ty = by_x_ax[it]; // this is top left point

            // check existence of top right point
            let by_x_rx = by_x.get(&rx).unwrap();
            let it_r = by_x_rx.binary_search(&ry).unwrap_err();
            let by_y_ty = by_y.get(&ty).unwrap();
            let it_t = by_y_ty.binary_search(&tx).unwrap_err();

            if it_r == by_x_rx.len() || it_t == by_y_ty.len() {
                continue;
            }
            if by_x_rx[it_r] != ty || by_y_ty[it_t] != rx {
                continue;
            }

            // dx/dy = down x/y,
            // ux/uy = up x/y
            // we +1 and -1 so we only get the inside (excluding border)
            let dx = ax + 1;
            let dy = ay + 1;
            let ux = rx - 1;
            let uy = ty - 1;

            let area = (to_original.get(&ty).unwrap() - to_original.get(&ay).unwrap()) as i64
                * (to_original.get(&rx).unwrap() - to_original.get(&ax).unwrap()) as i64;
            if dx <= ux && dy <= uy {
                // check if any point is in [dx, dy] [ux, uy] rectangle
                // if yes, then this isnt valid
                queries.push(Query::new(dx, ux, dy, uy, area));
            } else {
                ans = max(ans, area);
            }
        }

        queries.sort();

        // Mo's algorithm at play
        // Copied from CP-A
        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut ms: Multiset<i32> = Multiset::new();
        for q in queries.iter() {
            while cur_l > q.lx {
                cur_l -= 1;
                if let Some(vec) = by_x.get(&cur_l) {
                  ms.extend(vec.iter());
                }
            }
            while cur_r < q.rx {
                cur_r += 1;
                if let Some(vec) = by_x.get(&cur_r) {
                  ms.extend(vec.iter());
                }
            }
            while cur_l < q.lx {
              if let Some(vec) = by_x.get(&cur_l) {
                for &b in vec.iter() {
                  if let Some(index) = ms.find(&b) {
                    ms.remove(index);
                  }
                }
              }
              cur_l += 1;
            }
            while cur_r > q.rx {
              if let Some(vec) = by_x.get(&cur_r) {
                for &b in vec.iter() {
                  if let Some(index) = ms.find(&b) {
                    ms.remove(index);
                  }
                }
              }
              cur_r -= 1;
            }
            if let Some(it) = ms.lower_bound(&q.ly) {
                if *it <= q.ry {
                    continue; // there is a point that lies inside
                }
            }
            ans = max(ans, q.area);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let mut solution = Solution {};

    // Input the number of points
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str)?;
    let n: usize = n_str.trim().parse().unwrap();

    let mut x_coord: Vec<i32> = Vec::new();
    let mut y_coord: Vec<i32> = Vec::new();

    // Input the coordinates of the points
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let mut iter = line.trim().split_whitespace();

        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();

        x_coord.push(x);
        y_coord.push(y);
    }

    // Calculate the maximum rectangle area
    let max_area = Solution::maxRectangleArea(&mut x_coord, &mut y_coord);

    // Output the result
    println!("{}", max_area);

    Ok(())
}