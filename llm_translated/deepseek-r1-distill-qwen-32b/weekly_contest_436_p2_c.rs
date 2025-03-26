#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>
#include <math.h>

int* assignElements(int* groups, int groupsSize, int* elements, int elementsSize, int* returnSize) {
    int index[100005];
    for (int i = 0; i < 100005; i++) index[i] = INT_MAX;
    for (int i = 0; i < elementsSize; i++) {
        index[elements[i]] = fmin(index[elements[i]],i);
    }
    int* ans=(int*)malloc(groupsSize*sizeof(int));
    *returnSize=groupsSize;
    for(int i=0;i<groupsSize;i++){
        ans[i]=INT_MAX;
        for(int j=1;j<=sqrt(groups[i]);j++){
            if(groups[i]%j==0 &&(index[j]!=INT_MAX || index[groups[i]/j]!=INT_MAX)){
                ans[i]=fmin(ans[i],fmin(index[groups[i]/j],index[j]));
            }
        }
        if(ans[i]==INT_MAX) ans[i]=-1;
    }
    return ans;
}

int main() {
    // TODO: Add the base I/O interface here
    int n, m;
    scanf("%d %d", &n, &m);
    int* groups = (int*)malloc(n*sizeof(int));
    int* elements = (int*)malloc(m*sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &groups[i]);
    }
    for (int i = 0; i < m; i++) {
        scanf("%d", &elements[i]);
    }
    int returnSize;
    int* ans = assignElements(groups, n, elements, m, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d ", ans[i]);
    }
    printf("\n");
    free(groups);
    free(elements);
    free(ans);
    return 0;
}