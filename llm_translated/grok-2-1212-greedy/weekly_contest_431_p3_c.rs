// Translated from C to Rust using LLM
// Original: Weekly Contest 431 Problem 3

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

struct Solution;

impl Solution {
    // Placeholder for C++ method: int compare(const void* a, const void* b)
    fn compare(a: &str, b: &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: long max_ll(long long a, long long b)
    fn max_ll(a: i64, b: i64) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: long maximumCoins(int** coins, int coinsSize, int* coinsColSize, int k)
    fn maximumCoins(coins: &str, coinsSize: i32, coinsColSize: &str, k: i32) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: left boundary(coins[i][0])
    fn boundary() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: forward while(right < coinsSize && left < coinsSize)
    fn while(coinsSize: &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: int main()
    fn main() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: entry for(int i = 0; i < n; i++)
    fn for(i++: i32) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: function printf("%lld\n", maximumCoins(coins, n, coinsColSize, k)
    fn printf() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: memory for(int i = 0; i < n; i++)
    fn for(i++: i32) -> i32 {
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
Original C code:
// Problem: Weekly Contest 431 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Helper function to compare coins for sorting
int compare(const void* a, const void* b) {
    int* coin1 = *(int**)a;
    int* coin2 = *(int**)b;
    return coin1[0] - coin2[0]; // Compare by left boundary
}

// Function to find the maximum value between two long long values
long long max_ll(long long a, long long b) {
    return (a > b) ? a : b;
}

// Main solution function
long long maximumCoins(int** coins, int coinsSize, int* coinsColSize, int k) {
    // Sort coins by the left boundary (coins[i][0])
    qsort(coins, coinsSize, sizeof(int*), compare);
    
    // Calculate prefix sum of the coins' values
    long long* presum = (long long*)malloc((coinsSize + 1) * sizeof(long long));
    presum[0] = 0;
    for (int i = 1; i <= coinsSize; i++) {
        presum[i] = presum[i - 1] + (long long)(coins[i - 1][1] - coins[i - 1][0] + 1) * coins[i - 1][2];
    }
    
    long long ans = 0;
    int left = 0, right = 0;
    
    // First pass: moving right pointer forward
    while (right < coinsSize && left < coinsSize) {
        while (left < coinsSize && coins[right][1] - coins[left][0] + 1 > k) {
            long long tamp = k - (coins[right][0] - coins[left][0]);
            ans = max_ll(ans, tamp * coins[right][2] + presum[right] - presum[left]);
            left += 1;
        }
        if (left >= coinsSize) break;
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        right += 1;
    }
    
    // Second pass: moving left pointer backward
    left = coinsSize - 1;
    right = coinsSize - 1;
    while (right >= 0 && left >= 0) {
        while (right >= 0 && coins[right][1] - coins[left][0] + 1 > k) {
            long long tamp = k - (coins[right][1] - coins[left][1]);
            ans = max_ll(ans, tamp * coins[left][2] + presum[right + 1] - presum[left + 1]);
            right -= 1;
        }
        if (right < 0) break;
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        left -= 1;
    }
    
    free(presum);
    return ans;
}

int main() {
    int n, k;
    scanf("%d %d", &n, &k);
    
    // Allocate memory for coins array
    int** coins = (int**)malloc(n * sizeof(int*));
    int* coinsColSize = (int*)malloc(n * sizeof(int));
    
    // Initialize each coin entry
    for (int i = 0; i < n; i++) {
        coins[i] = (int*)malloc(3 * sizeof(int));
        coinsColSize[i] = 3;
        scanf("%d %d %d", &coins[i][0], &coins[i][1], &coins[i][2]);
    }
    
    // Call the solution function
    printf("%lld\n", maximumCoins(coins, n, coinsColSize, k));
    
    // Free allocated memory
    for (int i = 0; i < n; i++) {
        free(coins[i]);
    }
    free(coins);
    free(coinsColSize);
    
    return 0;
}

*/
