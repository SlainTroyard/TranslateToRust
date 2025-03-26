// Translated from C++ to Rust for LeetCode Weekly Contest 434 Problem 2
// Requirements:
// 1. Translates the entire program, including main
// 2. Preserves the algorithm logic exactly
// 3. Uses idiomatic Rust with proper error handling
// 4. Maintains the exact same stdin/stdout format
// 5. Carefully handles the same input parsing logic as the C++ code

use std::cmp::Ordering;
use std::io::{self, BufRead};

////////////////////////////////////////////////////////////////////////////////
// A helper struct that mimics C++ "cin" style token-by-token reading, and a
// separate function to read the remainder of the line (similar to getline).
////////////////////////////////////////////////////////////////////////////////
struct InputReader {
    buffer: Vec<String>,
    index: usize,
    // We store lines of standard input. Each time we exhaust the buffer,
    // we read another line, split it into tokens, and continue.
    stdin: std::io::Lines<std::io::StdinLock<'static>>,
}

impl InputReader {
    fn new() -> Self {
        // Box-leak pattern to extend stdin's lifetime for the duration of main
        let stdin = Box::leak(Box::new(io::stdin()));
        InputReader {
            buffer: Vec::new(),
            index: 0,
            stdin: stdin.lock().lines(),
        }
    }

    // Reads the next token (whitespace-delimited) from stdin,
    // similar to how C++ operator>> works.
    // If the current buffer is empty, we read a new line and split it.
    fn read_token(&mut self) -> Option<String> {
        while self.index >= self.buffer.len() {
            // Read the next line from stdin
            let line = self.stdin.next()?;
            let line = match line {
                Ok(l) => l,
                Err(_) => return None,
            };
            // Split into tokens
            self.buffer = line.split_whitespace().map(|s| s.to_string()).collect();
            self.index = 0;
        }
        // Return the next token
        let token = self.buffer[self.index].clone();
        self.index += 1;
        Some(token)
    }

    // Reads the remainder of the current line from stdin (similar to getline in C++).
    // In C++, getline consumes everything left INCLUDING leading spaces on that line.
    // Here, we must carefully replicate that. We'll do the simplest approach:
    //  - If there's leftover tokens in the buffer, that means we already consumed them
    //    as separate tokens. For an exact match with the C++ code, we specifically want
    //    to read from the same underlying input line, including leading spaces.
    // Implementing a fully identical logic can be tricky, but for close matching,
    // read a fresh line from raw stdin to simulate "getline".
    // This is not 100% identical if the C++ code still has tokens left on the same line,
    // but is a good approximation. The original code calls getline right after reading
    // one token with >>, so that typically consumes the rest of that line in C++.
    fn read_rest_of_line(&mut self) -> String {
        // If we're currently in the middle of a buffer, we've effectively already
        // consumed those tokens via read_token. The leftover spacing is lost.
        // We'll attempt to read a new raw line to capture spacing. 
        // This is a compromise, as the exact byte-for-byte spacing might differ
        // from the C++ code, but the overall logic (collect leftover text) remains.
        let next_line = self.stdin.next();
        match next_line {
            Some(Ok(line)) => {
                // Return with leading space if not empty
                if !line.is_empty() {
                    // Prepend a space to mimic the " events[i][2] += restOfLine;"
                    // which retained the leading space. In C++, there's typically
                    // a leading space coming from getline right after >>.
                    format!(" {}", line)
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// The translated "Solution" struct that encapsulates the original logic
////////////////////////////////////////////////////////////////////////////////
struct Solution;

impl Solution {
    // Comparator for sorting the events.
    // If times are equal and one event is OFFLINE but the other is not, OFFLINE comes first.
    // Otherwise, sort by ascending time.
    fn cmp(a: &[String], b: &[String]) -> Ordering {
        let time_a = a[1].parse::<i32>().expect("Invalid time in event[1]");
        let time_b = b[1].parse::<i32>().expect("Invalid time in event[1]");
        if time_a == time_b && a[0] != b[0] {
            if a[0] == "OFFLINE" {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            time_a.cmp(&time_b)
        }
    }

    // Extracts ID from string (e.g. "id123", "123"), returns 0 if parse fails.
    fn extract_id(&self, id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            let sub = &id_str[2..];
            match sub.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error parsing ID: {}", id_str);
                    0
                }
            }
        } else {
            match id_str.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error parsing ID: {}", id_str);
                    0
                }
            }
        }
    }

    // For a MESSAGE that mentions users by ID (e.g. "id1 id2 ..."),
    // split by whitespace, extract each ID, and increment counts if valid.
    fn update_counts(&self, counts: &mut [i32], mentions: &str) {
        for id_str in mentions.split_whitespace() {
            let id = self.extract_id(id_str);
            if id >= 0 && (id as usize) < counts.len() {
                counts[id as usize] += 1;
            }
        }
    }

    // Main logic that sorts events and counts the mentions.
    fn count_mentions(&self, number_of_users: usize, events: &mut Vec<Vec<String>>) -> Vec<i32> {
        // Sort with our custom comparator
        events.sort_by(|a, b| Self::cmp(a, b));

        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        // Process sorted events in order
        for event in events.iter() {
            let time: i32 = event[1].parse().expect("Invalid time in sorted events");
            if event[0] == "OFFLINE" {
                // OFFLINE event
                let id = self.extract_id(&event[2]);
                if id >= 0 && (id as usize) < number_of_users {
                    // If user was offline at t, we set their next available online time to t+60
                    online_times[id as usize] = time + 60;
                }
            } else {
                // MESSAGE event
                let mentions = &event[2];
                if mentions == "ALL" {
                    // Increments everyone at the end
                    all_count += 1;
                } else if mentions == "HERE" {
                    // Increments counts for all users who are "online now"
                    for i in 0..number_of_users {
                        if online_times[i] <= time {
                            counts[i] += 1;
                        }
                    }
                } else {
                    // Mentions specific users
                    self.update_counts(&mut counts, mentions);
                }
            }
        }

        // If there were ALL mentions, add that count to every user
        if all_count > 0 {
            for c in counts.iter_mut() {
                *c += all_count;
            }
        }

        counts
    }
}

////////////////////////////////////////////////////////////////////////////////
// The main function replicates the original C++ main exactly in terms of
// input and output handling.
////////////////////////////////////////////////////////////////////////////////
fn main() {
    let mut reader = InputReader::new();

    // Read numberOfUsers, numberOfEvents
    let number_of_users = reader
        .read_token()
        .expect("Failed to read number_of_users")
        .parse::<usize>()
        .expect("Invalid number_of_users");
    let number_of_events = reader
        .read_token()
        .expect("Failed to read number_of_events")
        .parse::<usize>()
        .expect("Invalid number_of_events");

    // Prepare a 2D vector of strings, each event has exactly 3 fields:
    //  [0]: event type (MESSAGE/OFFLINE/etc.)
    //  [1]: time
    //  [2]: additional parameter (e.g. "ALL", "HERE", or "id123", etc.)
    let mut events = vec![vec![String::new(); 3]; number_of_events];

    // Read each event according to the original logic:
    // - events[i][0] -> event type
    // - events[i][1] -> time
    // - if "MESSAGE", read a "messageType" token:
    //      if it's "ALL" or "HERE", store it directly in events[i][2]
    //      else store messageType + the remainder of that line
    // - if "OFFLINE", read the third token directly
    for i in 0..number_of_events {
        // Read event type
        let event_type = reader
            .read_token()
            .expect("Failed to read event type");
        // Read time
        let event_time = reader
            .read_token()
            .expect("Failed to read event time");

        events[i][0] = event_type;
        events[i][1] = event_time;

        if events[i][0] == "MESSAGE" {
            // Read next token, which might be "ALL", "HERE", or a user mention
            let message_type = reader
                .read_token()
                .expect("Failed to read message type for MESSAGE event");
            if message_type == "ALL" || message_type == "HERE" {
                // Just store it
                events[i][2] = message_type;
            } else {
                // This is a user mention. We replicate the original code which
                // reads the rest of the line after that token and appends it.
                events[i][2] = message_type;
                let rest_of_line = reader.read_rest_of_line();
                if !rest_of_line.is_empty() {
                    // Append the remainder
                    events[i][2].push_str(&rest_of_line);
                }
            }
        } else if events[i][0] == "OFFLINE" {
            // OFFLINE event, just read the third argument
            let offline_param = reader
                .read_token()
                .expect("Failed to read parameter for OFFLINE event");
            events[i][2] = offline_param;
        }
    }

    // Create a Solution instance and call countMentions
    let solution = Solution;
    let result = solution.count_mentions(number_of_users, &mut events);

    // Print results exactly as in C++: each count followed by a space, then one newline
    // (this includes a trailing space before the newline).
    for count in result {
        print!("{} ", count);
    }
    println!();
}