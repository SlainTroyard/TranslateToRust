// Translated from CPP to Rust using LLM
// Original: Weekly Contest 428 Problem 1

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

struct Solution;

impl Solution {
    // Placeholder for C++ method: int buttonWithLongestTime(vector<vector<int>>& events)
    fn buttonWithLongestTime(events: &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: int main()
    fn main() -> i32 {
        // Placeholder implementation
        0
    }

}

fn main() {
    // This is a placeholder implementation
    // In a real scenario, the LLM would translate the C++ I/O to Rust
    
    println!("Placeholder implementation. To get a proper translation, configure LLM API.");
}

/*
Original CPP code:
// Problem: Weekly Contest 428 Problem 1

#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int buttonWithLongestTime(vector<vector<int>>& events) {
        int idx = events[0][0], max_diff = events[0][1];
        for (int i = 1; i < events.size(); i++) {
            auto& p = events[i - 1];
            auto& q = events[i];
            int d = q[1] - p[1];
            if (d > max_diff || (d == max_diff && q[0] < idx)) {
                idx = q[0];
                max_diff = d;
            }
        }
        return idx;
    }
};

int main() {
    int n;
    cin >> n; // Number of events
    vector<vector<int>> events(n, vector<int>(2));
    
    for (int i = 0; i < n; ++i) {
        cin >> events[i][0] >> events[i][1]; // Read index and time
    }
    
    Solution solution;
    int result = solution.buttonWithLongestTime(events);
    cout << result << endl; // Output the result
    
    return 0;
}

*/
