use std::collections::{BTreeSet, HashMap, BTreeMap};
use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Query {
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
    area: i64,
}

struct Solution;

impl Solution {
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        let block_size = 512;
        let mut ans = -1;
        let n = x_coord.len();

        // Coordinate compression
        let all_coords: BTreeSet<_> = x_coord.iter().chain(y_coord.iter()).cloned().collect();
        let compressed_list: Vec<_> = all_coords.into_iter().collect();
        let to_compressed: HashMap<_, _> = compressed_list
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as i32))
            .collect();
        let to_original: Vec<i32> = compressed_list.clone();

        let mut x_coord = x_coord
            .into_iter()
            .map(|x| *to_compressed.get(&x).unwrap())
            .collect::<Vec<_>>();
        let mut y_coord = y_coord
            .into_iter()
            .map(|y| *to_compressed.get(&y).unwrap())
            .collect::<Vec<_>>();

        // Build by_x and by_y
        let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..n {
            let x = x_coord[i];
            let y = y_coord[i];
            by_x.entry(x).or_default().push(y);
            by_y.entry(y).or_default().push(x);
        }

        for ys in by_x.values_mut() {
            ys.sort_unstable();
        }
        for xs in by_y.values_mut() {
            xs.sort_unstable();
        }

        // Generate queries
        let mut queries = Vec::new();
        for i in 0..n {
            let ax = x_coord[i];
            let ay = y_coord[i];

            let ys_in_by_y = by_y.get(&ay).unwrap();
            let pos_rx = ys_in_by_y.partition_point(|&x| x <= ax);
            if pos_rx >= ys_in_by_y.len() {
                continue;
            }
            let rx = ys_in_by_y[pos_rx];

            let xs_in_by_x = by_x.get(&ax).unwrap();
            let pos_ty = xs_in_by_x.partition_point(|&y| y <= ay);
            if pos_ty >= xs_in_by_x.len() {
                continue;
            }
            let ty = xs_in_by_x[pos_ty];

            let rx_ys = by_x.get(&rx).unwrap();
            let pos_r = rx_ys.partition_point(|&y| y <= ay);
            if pos_r >= rx_ys.len() || rx_ys[pos_r] != ty {
                continue;
            }

            let ty_xs = by_y.get(&ty).unwrap();
            let pos_t = ty_xs.partition_point(|&x| x <= ax);
            if pos_t >= ty_xs.len() || ty_xs[pos_t] != rx {
                continue;
            }

            let dx = ax + 1;
            let dy = ay + 1;
            let ux = rx - 1;
            let uy = ty - 1;

            let area = (to_original[ty as usize] - to_original[ay as usize]) as i64
                * (to_original[rx as usize] - to_original[ax as usize]) as i64;

            if dx <= ux && dy <= uy {
                queries.push(Query {
                    lx: dx,
                    rx: ux,
                    ly: dy,
                    ry: uy,
                    area,
                });
            } else {
                ans = ans.max(area);
            }
        }

        // Sort queries using Mo's order
        queries.sort_by(|a, b| {
            let block_a = a.lx / block_size;
            let block_b = b.lx / block_size;
            (block_a, a.rx).cmp(&(block_b, b.rx))
        });

        // Mo's algorithm
        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut counts: BTreeMap<i32, usize> = BTreeMap::new();

        for q in queries {
            // Expand left
            while cur_l > q.lx {
                cur_l -= 1;
                let ys = by_x.get(&cur_l).unwrap();
                for &y in ys {
                    *counts.entry(y).or_insert(0) += 1;
                }
            }

            // Expand right
            while cur_r < q.rx {
                cur_r += 1;
                let ys = by_x.get(&cur_r).unwrap();
                for &y in ys {
                    *counts.entry(y).or_insert(0) += 1;
                }
            }

            // Contract left
            while cur_l < q.lx {
                let ys = by_x.get(&cur_l).unwrap();
                for &y in ys {
                    if let Some(count) = counts.get_mut(&y) {
                        *count -= 1;
                        if *count == 0 {
                            counts.remove(&y);
                        }
                    }
                }
                cur_l += 1;
            }

            // Contract right
            while cur_r > q.rx {
                let ys = by_x.get(&cur_r).unwrap();
                for &y in ys {
                    if let Some(count) = counts.get_mut(&y) {
                        *count -= 1;
                        if *count == 0 {
                            counts.remove(&y);
                        }
                    }
                }
                cur_r -= 1;
            }

            // Check for points in [q.ly, q.ry]
            if counts.range(q.ly..=q.ry).next().is_some() {
                continue;
            }

            ans = ans.max(q.area);
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut nums = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = nums.next().unwrap() as usize;
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    for _ in 0..n {
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        x_coord.push(x);
        y_coord.push(y);
    }

    let max_area = Solution::max_rectangle_area(x_coord, y_coord);
    println!("{}", max_area);
}