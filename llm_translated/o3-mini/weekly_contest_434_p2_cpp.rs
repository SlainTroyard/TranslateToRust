use std::io::{self, BufRead, Write};
use std::str::FromStr;

// Helper function to convert a string into an integer with error reporting.
fn parse_int<T: FromStr>(s: &str) -> Option<T> {
    s.parse::<T>().ok()
}

// Extract the numerical part of the id string.
// If the id starts with "id", then parse the remainder, otherwise try to parse the whole string.
fn extract_id(id_str: &str) -> i32 {
    if id_str.starts_with("id") {
        match id_str.get(2..) {
            Some(sub) => match sub.parse::<i32>() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error parsing ID: {}: {}", id_str, e);
                    0
                }
            },
            None => 0,
        }
    } else {
        match id_str.parse::<i32>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error parsing ID: {}: {}", id_str, e);
                0
            }
        }
    }
}

// Update counts for a given mentions string. The mentions is expected to be a space‐separated list of id strings.
fn update_counts(counts: &mut [i32], mentions: &str) {
    for id_str in mentions.split_whitespace() {
        let id = extract_id(id_str);
        if id >= 0 && (id as usize) < counts.len() {
            counts[id as usize] += 1;
        }
    }
}

// The Solution struct encapsulates our contest solution.
struct Solution;

impl Solution {
    // Count mentions given the number of users and list of events.
    fn count_mentions(&self, number_of_users: usize, events: &mut [Vec<String>]) -> Vec<i32> {
        // Custom comparator: sort events by time (converted from string) ascending.
        // If times are equal and event types differ, "OFFLINE" event comes first.
        events.sort_by(|a, b| {
            let time_a = a[1].parse::<i32>().unwrap_or(0);
            let time_b = b[1].parse::<i32>().unwrap_or(0);
            if time_a == time_b && a[0] != b[0] {
                // If event a is "OFFLINE", it should come before.
                if a[0] == "OFFLINE" {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else {
                time_a.cmp(&time_b)
            }
        });

        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in events.iter() {
            // Get time from event[1]
            let time = event[1].parse::<i32>().unwrap_or(0);

            if event[0] == "OFFLINE" {
                // Process OFFLINE event:
                // event[2] contains the id string of the user going offline.
                let id = extract_id(&event[2]);
                if id >= 0 && (id as usize) < number_of_users {
                    // Set user's online time to time + 60.
                    online_times[id as usize] = time + 60;
                }
                continue;
            }

            // For MESSAGE events, event[2] encodes who is mentioned.
            let mentions = &event[2];
            if mentions == "ALL" {
                all_count += 1;
            } else if mentions == "HERE" {
                // For HERE message, iterate over all users and check if they weren't marked as online after this event's time.
                for i in 0..number_of_users {
                    if online_times[i] <= time {
                        counts[i] += 1;
                    }
                }
            } else {
                // Otherwise, update counts with specific mentioned users.
                update_counts(&mut counts, mentions);
            }
        }

        // Add the "ALL" count to all users.
        if all_count > 0 {
            for count in counts.iter_mut() {
                *count += all_count;
            }
        }

        counts
    }
}

fn main() -> io::Result<()> {
    // Setup input reading from stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    // Read first line that contains numberOfUsers and numberOfEvents.
    reader.read_line(&mut line)?;
    let mut nums = line.split_whitespace();
    let number_of_users: usize = nums
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Failed to parse numberOfUsers");
    let number_of_events: usize = nums
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Failed to parse numberOfEvents");

    // Prepare the events vector: each event is a Vec<String> of length 3.
    // Read each event carefully, handling the possibility that the third field might contain spaces.
    let mut events: Vec<Vec<String>> = Vec::with_capacity(number_of_events);
    for _ in 0..number_of_events {
        let mut event: Vec<String> = Vec::with_capacity(3);
        line.clear();
        // Read a new line and split its initial tokens.
        // We read the line initially to get the first parts.
        reader.read_line(&mut line)?;
        let trimmed_line = line.trim_end_matches('\n');
        let mut parts = trimmed_line.split_whitespace();

        // The first token is the event type.
        let event_type = parts
            .next()
            .unwrap_or("")
            .to_string();
        event.push(event_type.clone());

        // The second token is the time.
        let time_str = parts
            .next()
            .unwrap_or("")
            .to_string();
        event.push(time_str);

        // Depending on the event type, we process the remaining information.
        if event_type == "MESSAGE" {
            // For MESSAGE events, read the next token first.
            let message_type = parts.next().unwrap_or("").to_string();

            // If message_type is "ALL" or "HERE", that's the whole third argument.
            if message_type == "ALL" || message_type == "HERE" {
                event.push(message_type);
            } else {
                // Otherwise, message_type may be the start of a list of IDs.
                // Reconstruct the rest of the line (there might be spaces)
                let mut mentions = message_type;
                // Check if there are additional tokens already in the line.
                if let Some(rest) = parts.clone().collect::<Vec<&str>>().join(" ").strip_prefix("") {
                    // In case there are extra tokens not consumed, append them.
                    let extra = parts.collect::<Vec<&str>>().join(" ");
                    if !extra.is_empty() {
                        mentions.push(' ');
                        mentions.push_str(&extra);
                    }
                }
                event.push(mentions);
            }
        } else if event_type == "OFFLINE" {
            // For OFFLINE events, the next token is the id string.
            let id_str = parts.next().unwrap_or("").to_string();
            event.push(id_str);
        } else {
            // In case of unknown event type or a format error, push an empty string.
            event.push(String::new());
        }

        events.push(event);
    }

    // Process the events with the solution.
    let solution = Solution;
    let mut events_mut = events; // events must be mutable for sorting.
    let counts = solution.count_mentions(number_of_users, &mut events_mut);

    // Write the result to stdout, separated by space.
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    for count in counts {
        write!(writer, "{} ", count)?;
    }
    writeln!(writer)?;
    Ok(())
}