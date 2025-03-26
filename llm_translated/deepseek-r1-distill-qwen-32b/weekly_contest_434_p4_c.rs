#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// 用于计算二进制表示中1的个数
int popcount(int n) {
    int count = 0;
    while (n) {
        count += n & 1;
        n >>= 1;
    }
    return count;
}

// 判断有向图是否有环
bool hasCycle(int sub, int g[26][26], int gSize[26]) {
    int color[26] = {0}; // 0: 未访问, 1: 正在访问, 2: 已访问完成
    
    // 使用非递归DFS检测环
    for (int start = 0; start < 26; start++) {
        // 如果节点不在子集中或已被完全访问，跳过
        if ((sub >> start & 1) == 0 || color[start] == 2) {
            continue;
        }
        
        // 使用栈代替递归
        int stack[26];
        int stackPos[26]; // 记录每个节点在其邻居列表中的当前位置
        int top = 0;
        
        stack[top] = start;
        stackPos[top] = 0;
        color[start] = 1; // 标记为正在访问
        
        while (top >= 0) {
            int x = stack[top];
            
            // 如果已经处理完所有邻居
            if (stackPos[top] >= gSize[x]) {
                color[x] = 2; // 标记为已访问完成
                top--;
                continue;
            }
            
            int y = g[x][stackPos[top]++];
            
            // 如果y不在当前子集中，跳过
            if ((sub >> y & 1) == 0) {
                continue;
            }
            
            // 如果y正在被访问，说明有环
            if (color[y] == 1) {
                return true;
            }
            
            // 如果y未被访问，将y入栈
            if (color[y] == 0) {
                color[y] = 1; // 标记为正在访问
                stack[++top] = y;
                stackPos[top] = 0;
            }
        }
    }
    
    return false;
}

/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *returnColumnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume caller calls free().
 */
int** supersequences(char** words, int wordsSize, int* returnSize, int** returnColumnSizes) {
    int all = 0, mask2 = 0;
    int g[26][26]; // 邻接表
    int gSize[26] = {0}; // 每个节点的邻居数量
    
    // 构建图和计算掩码
    for (int i = 0; i < wordsSize; i++) {
        int x = words[i][0] - 'a';
        int y = words[i][1] - 'a';
        
        all |= (1 << x) | (1 << y);
        
        if (x == y) {
            mask2 |= (1 << x);
        }
        
        g[x][gSize[x]++] = y;
    }
    
    // 计算mask1
    int mask1 = all ^ mask2;
    
    // 将validSubsets分配到堆上而不是栈上
    // 使用动态增长的数组更有效率
    int* validSubsets = NULL;
    int validSubsetsCapacity = 0;
    int validSubsetsCount = 0;
    int maxSize = 0;
    
    // 枚举mask1的所有子集
    int sub = mask1;
    do {
        int size = popcount(sub);
        
        if (size >= maxSize && !hasCycle(sub, g, gSize)) {
            if (size > maxSize) {
                maxSize = size;
                validSubsetsCount = 0;
            }
            
            // 确保数组有足够的空间
            if (validSubsetsCount >= validSubsetsCapacity) {
                validSubsetsCapacity = validSubsetsCapacity == 0 ? 16 : validSubsetsCapacity * 2;
                int* newArray = (int*)realloc(validSubsets, validSubsetsCapacity * sizeof(int));
                if (!newArray) {
                    // 内存分配失败，释放之前分配的内存
                    free(validSubsets);
                    *returnSize = 0;
                    return NULL;
                }
                validSubsets = newArray;
            }
            
            validSubsets[validSubsetsCount++] = sub;
        }
        
        // 计算mask1的下一个子集
        if (sub == 0) break;
        sub = (sub - 1) & mask1;
    } while (sub != mask1);
    
    // 分配返回结果的内存
    *returnSize = validSubsetsCount;
    int** result = NULL;
    
    if (validSubsetsCount > 0) {
        result = (int**)malloc(validSubsetsCount * sizeof(int*));
        if (!result) {
            free(validSubsets);
            *returnSize = 0;
            return NULL;
        }
        
        *returnColumnSizes = (int*)malloc(validSubsetsCount * sizeof(int));
        if (!(*returnColumnSizes)) {
            free(validSubsets);
            free(result);
            *returnSize = 0;
            return NULL;
        }
        
        // 构建结果
        for (int i = 0; i < validSubsetsCount; i++) {
            int sub = validSubsets[i];
            result[i] = (int*)malloc(26 * sizeof(int));
            if (!result[i]) {
                // 内存分配失败，释放之前分配的内存
                for (int j = 0; j < i; j++) {
                    free(result[j]);
                }
                free(*returnColumnSizes);
                free(result);
                free(validSubsets);
                *returnSize = 0;
                return NULL;
            }
            
            (*returnColumnSizes)[i] = 26;
            
            // 计算每个字母的出现次数
            for (int j = 0; j < 26; j++) {
                result[i][j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
            }
        }
    }
    
    // 释放临时内存
    free(validSubsets);
    
    return result;
}

int main() {
    // 读取输入
    int n;
    if (scanf("%d", &n) != 1) {
        fprintf(stderr, "Error reading input for n\n");
        return 1;
    }
    
    // 分配内存并读取字符串数组
    char** words = (char**)malloc(n * sizeof(char*));
    if (!words) {
        fprintf(stderr, "Memory allocation failed for words array\n");
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        words[i] = (char*)malloc(3 * sizeof(char)); // 每个字符串长度为2，加上'\0'
        if (!words[i]) {
            fprintf(stderr, "Memory allocation failed for words[%d]\n", i);
            // 释放已分配的内存
            for (int j = 0; j < i; j++) {
                free(words[j]);
            }
            free(words);
            return 1;
        }
        
        if (scanf("%2s", words[i]) != 1) {
            fprintf(stderr, "Error reading input for words[%d]\n", i);
            // 释放已分配的内存
            for (int j = 0; j <= i; j++) {
                free(words[j]);
            }
            free(words);
            return 1;
        }
    }
    
    // 调用函数计算结果
    int returnSize;
    int* returnColumnSizes;
    int** result = supersequences(words, n, &returnSize, &returnColumnSizes);
    
    // 输出结果
    for (int i = 0; i < returnSize; i++) {
        for (int j = 0; j < returnColumnSizes[i]; j++) {
            printf("%d ", result[i][j]);
        }
        printf("\n");
    }
    
    // 释放内存
    for (int i = 0; i < returnSize; i++) {
        free(result[i]);
    }
    free(result);
    free(returnColumnSizes);
    
    for (int i = 0; i < n; i++) {
        free(words[i]);
    }
    free(words);
    
    return 0;
}