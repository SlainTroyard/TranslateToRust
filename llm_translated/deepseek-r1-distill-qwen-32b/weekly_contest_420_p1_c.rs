// Problem: Weekly Contest 420 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char** stringSequence(char* target, int* returnSize) {
    * returnSize = 0;
    for (char * t = target; * t; ++ t) * returnSize += * t - 96;
    char ** ans = (char **)calloc(* returnSize, sizeof(char *)); 
    int i = 0, l = 0, size = sizeof(char);
    for (char * t = target; * t; ++ t, ++ l) 
        for (char c = 'a'; c <= * t; ++ c) {
            ans[i] = (char *)calloc(l + 2, size);
            strncpy(ans[i], target, l);
            ans[i ++][l] = c;
        }
    return ans;
}


int main() {
    char target[100];
    scanf("%s", target);
    int returnSize;
    char ** ans = stringSequence(target, & returnSize);
    for (int i = 0; i < returnSize; ++ i) {
        printf("%s ", ans[i]);
        free(ans[i]);
    }
    free(ans);
    printf("\n");
    return 0;
}