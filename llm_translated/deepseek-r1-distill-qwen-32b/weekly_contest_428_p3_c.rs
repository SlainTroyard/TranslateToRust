#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

int beautifulSplits(int* nums, int numsSize) {
    int res=0;
    int cnt0[numsSize],kmpcnt=0;
    cnt0[0]=0;
    for(int i=1;i<numsSize;i++){
        while(kmpcnt >0  && nums[i] != nums[kmpcnt])
        {
            kmpcnt=cnt0[kmpcnt-1];
        }
        if(nums[i] == nums[kmpcnt]) kmpcnt++;
        cnt0[i]=kmpcnt;
        if(i%2 && 0==(((i+1)/2) %(i+1-kmpcnt))) {
            res+=numsSize-i-1;
        }
    }
    for(int i=1;i<numsSize;i++){
        int cnt[numsSize-i],end=numsSize;
        kmpcnt=0;
        cnt[0]=0;
        if(2*i <numsSize && 0==(i%(2*i-cnt0[2*i-1]))) end=fmin(end,3*i);
        for(int j=i+1;j<end;j++){
            while(kmpcnt >0  && nums[j] != nums[i+kmpcnt])
            {
                kmpcnt=cnt[kmpcnt-1];
    		}
            if(nums[j] == nums[i+kmpcnt]) kmpcnt++;
            cnt[j-i]=kmpcnt;
            if((0==(j-i+1)%2)&& (0==((j-i+1)/2)%(j-i+1-kmpcnt))) {
                int len=j-i+1,div=i-1+len/2;
                if(len==i*2 && (0==(i%(div+1-cnt0[div])))) break;
                res+=1;
            }
        }
    }
    return res;
}

int main() {
    int n;

    // Input: size of the array
    scanf("%d", &n);

    // Input: array elements
    int* nums = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }

    // Calculate and print the result
    int result = beautifulSplits(nums, n);
    printf("%d\n", result);

    // Free allocated memory
    free(nums);

    return 0;
}