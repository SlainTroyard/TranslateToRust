#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define max(base, n) (n)>base?(n):base

int subarraySum(int* nums, int numsSize) {
    int i, ans = 0;
    int sums[numsSize+1];
    sums[0] = 0;
    for(i = 0; i< numsSize; i++){
        sums[i+1] = nums[i] + sums[i];
        ans += sums[i+1] - sums[max(0, i-nums[i])];
    }
    return ans;
}

int main() {
    // TODO: Add the base I/O interface here
    int n;
    scanf("%d", &n);
    int* nums = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }
    printf("%d\n", subarraySum(nums, n));
    free(nums);
    return 0;
}