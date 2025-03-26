// Translated from CPP to Rust using LLM
// Original: Weekly Contest 417 Problem 2

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

struct Solution;

impl Solution {
    // Placeholder for C++ method: long f(string& word, int k)
    fn f(word: &str, k: i32) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: long countOfSubstrings(string word, int k)
    fn countOfSubstrings(word: &str, k: i32) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: return f(word, k)
    fn f() -> i32 {
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
// Problem: Weekly Contest 417 Problem 2
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
    const int VOWEL_MASK = 1065233;

    long long f(string& word, int k) {
        long long ans = 0;
        int cnt1['u' - 'a' + 1]{};
        int size1 = 0; // 元音种类数
        int cnt2 = 0;
        int left = 0;
        for (char b : word) {
            b -= 'a';
            if (VOWEL_MASK >> b & 1) {
                if (cnt1[b]++ == 0) {
                    size1++;
                }
            } else {
                cnt2++;
            }
            while (size1 == 5 && cnt2 >= k) {
                char out = word[left] - 'a';
                if (VOWEL_MASK >> out & 1) {
                    if (--cnt1[out] == 0) {
                        size1--;
                    }
                } else {
                    cnt2--;
                }
                left++;
            }
            ans += left;
        }
        return ans;
    }

public:
    long long countOfSubstrings(string word, int k) {
        return f(word, k) - f(word, k + 1);
    }
};

int main() {
    int wordSize, k;
    cin >> wordSize;
    string word;
    cin >> word;
    cin >> k;
    Solution sol;
    cout << sol.countOfSubstrings(word, k) << endl;
    return 0;
}

*/
