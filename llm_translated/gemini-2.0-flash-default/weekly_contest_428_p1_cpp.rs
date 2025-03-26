use std::io;
use std::io::Read;
use std::str::FromStr;

fn button_with_longest_time(events: &Vec<Vec<i32>>) -> i32 {
    let mut idx = events[0][0];
    let mut max_diff = events[0][1];

    for i in 1..events.len() {
        let p = &events[i - 1];
        let q = &events[i];
        let d = q[1] - p[1];
        if d > max_diff || (d == max_diff && q[0] < idx) {
            idx = q[0];
            max_diff = d;
        }
    }
    idx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let n: i32 = lines.next().unwrap().parse().unwrap(); // Number of events

    let mut events: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let a: i32 = nums.next().unwrap().parse().unwrap();
        let b: i32 = nums.next().unwrap().parse().unwrap();
        events.push(vec![a, b]);
    }

    let result = button_with_longest_time(&events);
    println!("{}", result); // Output the result
}