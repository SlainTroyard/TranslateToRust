use std::collections::{BTreeSet, HashMap};
use std::io;
use std::cmp::Ordering;

const BLOCK_SIZE: i32 = 512;

#[derive(Debug, Eq)]
struct Query {
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
    area: i64,
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        let key1 = (self.lx / BLOCK_SIZE, self.rx);
        let key2 = (other.lx / BLOCK_SIZE, other.rx);
        key1.cmp(&key2)
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx == other.lx && self.rx == other.rx && self.ly == other.ly && self.ry == other.ry
    }
}

struct Solution;

impl Solution {
    fn max_rectangle_area(&self, x_coord: &mut Vec<i32>, y_coord: &mut Vec<i32>) -> i64 {
        let n = x_coord.len() as i32;
        if n == 0 {
            return 0;
        }

        // Coordinate compression
        let mut all_coords: Vec<i32> = x_coord.iter().cloned()
            .chain(y_coord.iter().cloned())
            .collect();
        all_coords.sort();
        all_coords.dedup();

        let mut to_compressed = HashMap::new();
        let mut to_original = HashMap::new();
        let mut lst = 0;
        for &a in &all_coords {
            to_compressed.insert(a, lst);
            to_original.insert(lst, a);
            lst += 1;
        }

        for i in 0..x_coord.len() {
            x_coord[i] = *to_compressed.get(&x_coord[i]).unwrap();
        }
        for i in 0..y_coord.len() {
            y_coord[i] = *to_compressed.get(&y_coord[i]).unwrap();
        }

        // Build byX and byY
        let mut by_x = HashMap::new();
        let mut by_y = HashMap::new();
        for i in 0..n as usize {
            let x = x_coord[i];
            let y = y_coord[i];
            by_x.entry(x).or_insert_with(Vec::new).push(y);
            by_y.entry(y).or_insert_with(Vec::new).push(x);
        }

        // Sort the vectors
        for x in by_x.values_mut() {
            x.sort();
        }
        for y in by_y.values_mut() {
            y.sort();
        }

        // Helper function for upper_bound
        fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
            let mut low = 0;
            let mut high = arr.len();
            while low < high {
                let mid = (low + high) / 2;
                if arr[mid] > *target {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            low
        }

        let mut queries = Vec::new();
        let mut ans = -1;

        for i in 0..n as usize {
            let ax = x_coord[i];
            let ay = y_coord[i];

            // Find rx and ry
            if let Some(by_y_ay) = by_y.get(&ay) {
                let it = upper_bound(by_y_ay, &ax);
                if it >= by_y_ay.len() {
                    continue;
                }
                let rx = by_y_ay[it];
                let ry = ay;

                // Find tx and ty
                if let Some(by_x_ax) = by_x.get(&ax) {
                    let it = upper_bound(by_x_ax, &ay);
                    if it >= by_x_ax.len() {
                        continue;
                    }
                    let tx = ax;
                    let ty = by_x_ax[it];

                    // Check top-right and bottom-left
                    if let Some(by_x_rx) = by_x.get(&rx) {
                        let it_r = upper_bound(by_x_rx, &ry);
                        if it_r >= by_x_rx.len() {
                            continue;
                        }
                        if by_x_rx[it_r] != ty {
                            continue;
                        }

                        if let Some(by_y_ty) = by_y.get(&ty) {
                            let it_t = upper_bound(by_y_ty, &tx);
                            if it_t >= by_y_ty.len() {
                                continue;
                            }
                            if by_y_ty[it_t] != rx {
                                continue;
                            }

                            let dx = ax + 1;
                            let dy = ay + 1;
                            let ux = rx - 1;
                            let uy = ty - 1;

                            let area = (to_original.get(&ty).unwrap() - to_original.get(&ay).unwrap()) as i64
                                * (to_original.get(&rx).unwrap() - to_original.get(&ax).unwrap()) as i64;

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
                    }
                }
            }
        }

        // Sort queries
        queries.sort();

        // Mo's algorithm
        let mut cur_l = 0;
        let mut cur_r = -1;
        let mut ms = BTreeSet::new();

        for q in queries {
            // Expand to the left
            while cur_l > q.lx {
                cur_l -= 1;
                if let Some(ys) = by_x.get(&cur_l) {
                    for y in ys {
                        ms.insert(*y);
                    }
                }
            }
            // Expand to the right
            while cur_r < q.rx {
                cur_r += 1;
                if let Some(ys) = by_x.get(&cur_r) {
                    for y in ys {
                        ms.insert(*y);
                    }
                }
            }
            // Shrink from the left
            while cur_l < q.lx {
                if let Some(ys) = by_x.get(&cur_l) {
                    for y in ys {
                        ms.remove(y);
                    }
                }
                cur_l += 1;
            }
            // Shrink from the right
            while cur_r > q.rx {
                if let Some(ys) = by_x.get(&cur_r) {
                    for y in ys {
                        ms.remove(y);
                    }
                }
                cur_r -= 1;
            }

            // Check if any point exists in [ly, ry]
            if ms.range(q.ly..=q.ry).next().is_none() {
                ans = ans.max(q.area);
            }
        }

        ans
    }
}

fn read_i32() -> io::Result<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn main() -> io::Result<()> {
    let n = read_i32()?;
    let mut x_coord = Vec::with_capacity(n as usize);
    let mut y_coord = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let x = read_i32()?;
        let y = read_i32()?;
        x_coord.push(x);
        y_coord.push(y);
    }

    let solution = Solution;
    let max_area = solution.max_rectangle_area(&mut x_coord, &mut y_coord);

    println!("{}", max_area);

    Ok(())
}