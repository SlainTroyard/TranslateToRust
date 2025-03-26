use std::io;
use std::collections::{BTreeSet, BTreeMap, HashMap};

const BLOCK_SIZE: i32 = 512;

struct Query {
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
    area: i64,
}

pub fn max_rectangle_area(x_coord: &mut Vec<i32>, y_coord: &mut Vec<i32>) -> i64 {
    let n = x_coord.len();
    if n == 0 {
        return -1;
    }

    let mut ans = -1i64;

    // Coordinate compression
    let mut coords: BTreeSet<i32> = BTreeSet::new();
    for &x in x_coord.iter() {
        coords.insert(x);
    }
    for &y in y_coord.iter() {
        coords.insert(y);
    }
    let sorted_coords: Vec<i32> = coords.into_iter().collect();

    let mut to_compressed = HashMap::new();
    let mut to_original = HashMap::new();
    for (idx, &val) in sorted_coords.iter().enumerate() {
        to_compressed.insert(val, idx as i32);
        to_original.insert(idx as i32, val);
    }

    let mut compressed_x: Vec<i32> = Vec::with_capacity(n);
    let mut compressed_y: Vec<i32> = Vec::with_capacity(n);
    for &x in x_coord.iter() {
        compressed_x.push(*to_compressed.get(&x).unwrap());
    }
    for &y in y_coord.iter() {
        compressed_y.push(*to_compressed.get(&y).unwrap());
    }

    let mut byX: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut byY: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..n {
        let x = compressed_x[i];
        let y = compressed_y[i];
        byX.entry(x).or_insert_with(Vec::new).push(y);
        byY.entry(y).or_insert_with(Vec::new).push(x);
    }

    for vec in byX.values_mut() {
        vec.sort_unstable();
    }
    for vec in byY.values_mut() {
        vec.sort_unstable();
    }

    let mut queries = Vec::new();

    for i in 0..n {
        let ax = compressed_x[i];
        let ay = compressed_y[i];

        // Find rx (right x) from byY[ay]
        let byY_ay = byY.get(&ay).unwrap();
        let pos_rx = byY_ay.binary_search(&ax).unwrap_or_else(|x| x);
        if pos_rx >= byY_ay.len() {
            continue;
        }
        let rx = byY_ay[pos_rx];

        // Find ty (top y) from byX[ax]
        let byX_ax = byX.get(&ax).unwrap();
        let pos_ty = byX_ax.binary_search(&ay).unwrap_or_else(|x| x);
        if pos_ty >= byX_ax.len() {
            continue;
        }
        let ty = byX_ax[pos_ty];

        // Check top-right point (rx, ty)
        let ry = ay;
        let byX_rx = byX.get(&rx).unwrap();
        let pos_tr = byX_rx.binary_search(&ry).unwrap_or_else(|x| x);
        if pos_tr >= byX_rx.len() {
            continue;
        }
        let tr_val = byX_rx[pos_tr];
        if tr_val != ty {
            continue;
        }

        // Check byY[ty] has rx after tx (ax)
        let tx = ax;
        let byY_ty = byY.get(&ty).unwrap();
        let pos_tt = byY_ty.binary_search(&tx).unwrap_or_else(|x| x);
        if pos_tt >= byY_ty.len() {
            continue;
        }
        let tt_val = byY_ty[pos_tt];
        if tt_val != rx {
            continue;
        }

        // Now check dx <= ux and dy <= uy
        let dx = ax + 1;
        let ux = rx - 1;
        let dy = ay + 1;
        let uy = ty - 1;

        if dx > ux || dy > uy {
            let original_ay = to_original[&ay];
            let original_ty = to_original[&ty];
            let original_ax = to_original[&ax];
            let original_rx = to_original[&rx];
            let area = (original_ty - original_ay) as i64 * (original_rx - original_ax) as i64;
            ans = ans.max(area);
            continue;
        }

        // Create query
        let original_ay = to_original[&ay];
        let original_ty = to_original[&ty];
        let original_ax = to_original[&ax];
        let original_rx = to_original[&rx];
        let area = (original_ty - original_ay) as i64 * (original_rx - original_ax) as i64;
        queries.push(Query {
            lx: dx,
            rx: ux,
            ly: dy,
            ry: uy,
            area,
        });
    }

    // Sort queries
    queries.sort_by(|a, b| {
        let a_key = (a.lx / BLOCK_SIZE, a.rx);
        let b_key = (b.lx / BLOCK_SIZE, b.rx);
        a_key.cmp(&b_key)
    });

    // Mo's algorithm
    let mut cur_l = 0;
    let mut cur_r = -1;
    let mut ms: BTreeSet<i32> = BTreeSet::new();

    for q in queries.iter() {
        // Adjust cur_l and cur_r to match q's lx and rx
        while cur_l > q.lx {
            cur_l -= 1;
            if let Some(ys) = byX.get(&cur_l) {
                for &y in ys {
                    ms.insert(y);
                }
            }
        }
        while cur_r < q.rx {
            cur_r += 1;
            if let Some(ys) = byX.get(&(cur_r)) {
                for &y in ys {
                    ms.insert(y);
                }
            }
        }
        while cur_l < q.lx {
            if let Some(ys) = byX.get(&cur_l) {
                for &y in ys {
                    ms.remove(&y);
                }
            }
            cur_l += 1;
        }
        while cur_r > q.rx {
            if let Some(ys) = byX.get(&(cur_r)) {
                for &y in ys {
                    ms.remove(&y);
                }
            }
            cur_r -= 1;
        }

        // Check if the current query's rectangle is valid
        if !ms.range(q.ly..=q.ry).next().is_some() {
            ans = ans.max(q.area);
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    for line in lines {
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        x_coord.push(x);
        y_coord.push(y);
    }

    let max_area = max_rectangle_area(&mut x_coord, &mut y_coord);
    println!("{}", max_area);
}