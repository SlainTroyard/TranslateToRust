#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int maxGoodNumber(int* nums, int numsSize) {
    int nums1 = 0;
    int num2 = 0;
    int num3 = 0;
    for (int i = 0; i < numsSize; i++) {
        for (int j = 0; j < 7; j++) {
            if (i == 0) {
                int s = (nums[i] << j) & 64;
                if (s == 0)
                    nums1++;
                else
                    break;
            } else if (i == 1) {
                int s = (nums[i] << j) & 64;
                if (s == 0)
                    num2++;
                else
                    break;
            } else if (i == 2) {
                int s = (nums[i] << j) & 64;
                if (s == 0)
                    num3++;
                else
                    break;
            }
        }
    }
    int* times = (int*)calloc(3, sizeof(int));
    times[0] = nums1;
    times[1] = num2;
    times[2] = num3;
    int* store = (int*)calloc(3, sizeof(int));
    store[1] = 1;
    store[2] = 2;
    for (int i = 1; i <= 2; i++) {
        for (int j = 0; j <= 1; j++) {
            int value1 = (nums[store[j]] << (7 - times[store[j + 1]])) +
                         nums[store[j + 1]];
            int value2 =
                (nums[store[j + 1]] << (7 - times[store[j]])) + nums[store[j]];
            if (value2 >= value1) {
                int temp = store[j];
                store[j] = store[j + 1];
                store[j + 1] = temp;
            }
        }
    }
    return (nums[store[0]] << (14 - times[store[1]] - times[store[2]])) +
           (nums[store[1]] << (7 - times[store[2]])) + nums[store[2]];
}

int main() {
    int numSize;
    scanf("%d", &numSize);
    int* nums = (int*)calloc(numSize, sizeof(int));
    for (int i = 0; i < numSize; i++) {
        scanf("%d", &nums[i]);
    }
    printf("%d\n", maxGoodNumber(nums, numSize));
    free(nums);
    return 0;
}