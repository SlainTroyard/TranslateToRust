struct Solution;

impl Solution {
    fn get(&self, points: &Vec<(i32, i32)>) -> i32 {
        let mut max_area = -1;
        for i in 0..points.len() - 3 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];
            let (x3, y3) = points[i + 2];
            let (x4, y4) = points[i + 3];
            
            if x1 == x2 && x3 == x4 && y1 == y3 && y2 == y4 {
                let area = (x3 - x1) * (y2 - y1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }

    fn max_rectangle_area(&self, points: &mut Vec<(i32, i32)>) -> i32 {
        let n = points.len();
        if n < 4 {
            return -1;
        }
        points.sort();
        let mut max_area = -1;
        
        for i in 0..n - 3 {
            let mut rect_points = Vec::new();
            rect_points.push(points[i]);
            rect_points.push(points[i + 1]);
            
            let mut j = i + 2;
            while j < n - 2 {
                if points[j].1 > points[i + 1].1 || points[j].1 < points[i].1 {
                    j += 1;
                } else {
                    break;
                }
            }
            
            if j < n - 1 {
                rect_points.push(points[j]);
                rect_points.push(points[j + 1]);
                
                let area = self.get(&rect_points);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        
        max_area
    }
}

fn read_points() -> Vec<(i32, i32)> {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = tokens[0] as usize;
    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        let x = tokens[1 + 2 * i];
        let y = tokens[2 + 2 * i];
        points.push((x, y));
    }
    points
}

fn main() {
    let mut solution = Solution;
    let mut points = read_points();
    let result = solution.max_rectangle_area(&mut points);
    println!("{}", result);
}