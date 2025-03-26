#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int* zigzagTraversal(int** grid, int gridSize, int* gridColSize, int* returnSize) {
    int * ans = (int *)calloc((* gridColSize * gridSize + 1) >> 1, sizeof(int)), r = 0, c, c1 = * gridColSize - 1 - (* gridColSize & 1); 
    for (* returnSize = 0; r < gridSize; ++ r) 
        if (r & 1) 
            for (c = c1; c >= 0           ; c -= 2) 
                ans[(* returnSize) ++] = grid[r][c];
        else 
            for (c = 0 ; c < * gridColSize; c += 2) 
                ans[(* returnSize) ++] = grid[r][c];
    return ans;
}

int main() {
    int gridSize, gridColSize, returnSize;
    scanf("%d %d", &gridSize, &gridColSize);
    int ** grid = (int **)malloc(gridSize * sizeof(int *));
    for (int i = 0; i < gridSize; i ++) {
        grid[i] = (int *)malloc(gridColSize * sizeof(int));
        for (int j = 0; j < gridColSize; j ++) {
            scanf("%d", &grid[i][j]);
        }
    }
    int * ans = zigzagTraversal(grid, gridSize, &gridColSize, &returnSize);
    for (int i = 0; i < returnSize; i ++) {
        printf("%d ", ans[i]);
    }
    free(ans);
    for (int i = 0; i < gridSize; i ++) {
        free(grid[i]);
    }
    free(grid);
    return 0;
}