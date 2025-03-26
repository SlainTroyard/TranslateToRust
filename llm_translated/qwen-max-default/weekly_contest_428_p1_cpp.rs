use std::io;

fn main() {
    // Read the number of events from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Initialize the events vector
    let mut events: Vec<Vec<i32>> = vec![vec![0; 2]; n];

    // Read each event's index and time
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Please type a number!"))
            .collect();
        events[i] = vec![nums[0], nums[1]];
    }

    // Create an instance of Solution and get the result
    let solution = Solution;
    let result = solution.button_with_longest_time(&events);

    // Output the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Finds the button with the longest time between consecutive events.
    ///
    /// # Arguments
    ///
    /// * `events` - A vector of vectors, where each inner vector contains two integers:
    ///   - The first integer is the button index.
    ///   - The second integer is the time of the event.
    ///
    /// # Returns
    ///
    /// * The index of the button with the longest time between consecutive events.
    pub fn button_with_longest_time(events: &Vec<Vec<i32>>) -> i32 {
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
}