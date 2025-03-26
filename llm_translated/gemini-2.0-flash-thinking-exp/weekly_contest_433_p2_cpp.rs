// Translated from CPP to Rust using LLM
// Original: Weekly Contest 433 Problem 2

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

struct Solution;

impl Solution {
    // Placeholder for C++ method: long pow(long long x, int n)
    fn pow(x: i64, n: i32) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: long comb(int n, int m)
    fn comb(n: i32, m: i32) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: int minMaxSums(vector<int>& nums, int k)
    fn minMaxSums(nums: &str, k: i32) -> i32 {
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
// Problem: Weekly Contest 433 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
using namespace std;

const int MOD = 1'000'000'007;
const int MX = 100'000;

long long F[MX]; // F[i] = i!
long long INV_F[MX]; // INV_F[i] = i!^-1

long long pow(long long x, int n) {
    long long res = 1;
    for (; n; n /= 2) {
        if (n % 2) {
            res = res * x % MOD;
        }
        x = x * x % MOD;
    }
    return res;
}

auto init = [] {
    F[0] = 1;
    for (int i = 1; i < MX; i++) {
        F[i] = F[i - 1] * i % MOD;
    }

    INV_F[MX - 1] = pow(F[MX - 1], MOD - 2);
    for (int i = MX - 1; i; i--) {
        INV_F[i - 1] = INV_F[i] * i % MOD;
    }
    return 0;
}();

long long comb(int n, int m) {
    return m > n ? 0 : F[n] * INV_F[m] % MOD * INV_F[n - m] % MOD;
}

class Solution {
public:
    int minMaxSums(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        int n = nums.size();
        long long ans = 0, s = 1;
        for (int i = 0; i < n; i++) {
            ans = (ans + s * (nums[i] + nums[n - 1 - i])) % MOD;
            s = (s * 2 - comb(i, k - 1) + MOD) % MOD;
        }
        return ans;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n, k;
    cin >> n >> k;
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    Solution sol;
    cout << sol.minMaxSums(nums, k) << endl;
    return 0;
}

*/
