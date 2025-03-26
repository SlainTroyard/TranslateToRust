// Translated from C to Rust using LLM
// Original: Weekly Contest 421 Problem 1

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

struct Solution;

impl Solution {
    // Placeholder for C++ method: long gcd(long long a,long long b)
    fn gcd(a: i64, b: i64) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: long lcm(long long a,long long b)
    fn lcm(a: i64, b: i64) -> i64 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: long maxScore(int* nums, int numsSize)
    fn maxScore(nums: &str, numsSize: i32) -> i64 {
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
Original C code:
// Problem: Weekly Contest 421 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
#include <limits.h>

long long gcd(long long a,long long b){
    long long c=a%b;
    while(c){
        a=b,b=c;
        c=a%b;
    }
    return b;
}
long long lcm(long long a,long long b){
    return (a/gcd(a,b))*b;
}
long long maxScore(int* nums, int numsSize) {
    if(numsSize==1)return nums[0]*nums[0];
    long long lcms[numsSize],gcds[numsSize];
    lcms[numsSize-1]=gcds[numsSize-1]=nums[numsSize-1];
    for(int i=numsSize-2;i>=0;i--){
        lcms[i]=lcm(nums[i],lcms[i+1]);
        gcds[i]=gcd(nums[i],gcds[i+1]);
    }
    long long ans=lcms[0]*gcds[0];
    ans=fmax(ans,lcms[1]*gcds[1]);
    long long prelcm=nums[0],pregcd=nums[0];
    for(int i=1;i<numsSize-1;i++){
        ans=fmax(gcd(pregcd,gcds[i+1])*lcm(prelcm,lcms[i+1]),ans);
        prelcm=lcm(prelcm,nums[i]);
        pregcd=gcd(pregcd,nums[i]);
    }
    ans=fmax(ans,prelcm*pregcd);
    return ans;
}

int main() {
    int numsSize;
    scanf("%d", &numsSize);
    int nums[numsSize];
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    printf("%lld\n", maxScore(nums, numsSize));
    return 0;
}

*/
