#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
#include <limits.h>

long long zhuanhuan(int *point,int side)
{
    if(point[0]>=point[1]) return point[0]+point[1];
    else 
    {
        long long m=side;
        return 4*m-(point[0]+point[1]);
    }
}
int compar(const void *a, const void *b) {
    long long val_a = *(const long long *)a;
    long long val_b = *(const long long *)b;

    if (val_a < val_b) return -1;
    if (val_a > val_b) return 1;
    return 0;
}
long long search(long long val,long long *r,int pointsSize,int side)
{
    long long m=side;
    long long val1=val%(4*m);
    if(val1>r[pointsSize-1]) return val-val1+4*m+r[0];
    if(val1<=r[0]) return val-val1+r[0];
    int min=0,max=pointsSize-1;
    while(min<max-1)
    {
        int s=(max+min)/2;
        if(r[s]>=val1) max=s;
        else min=s;
    }
    return r[max]+val-val1;
}
bool build(long long * r,int s,int k,int side,int pointsSize)
{
    long long sum,th,val,max_th;
    long long m=side;
    for(int i=0;i<pointsSize-1;i++)
    {
        sum=1;
        th=r[i];
        max_th=r[i]+4*m-s;
        while(th<=max_th)
        {
            if(sum==k) return true;
            val=(th+s);
            th=search(val,r,pointsSize,side);
            sum++;
        }
    }
    return false;
}
int maxDistance(int side, int** points, int pointsSize, int* pointsColSize, int k) {
    long long *r=malloc(sizeof(long long)*pointsSize);
    for(int i=0;i<pointsSize;i++)
    {
        r[i]=zhuanhuan(points[i],side);
    }
    qsort(r,pointsSize,sizeof(long long),compar);
    int min=1,max=side+1,s;
    while(min<max-1)
    {
        s=(min+max)/2;
        if(build(r,s,k,side,pointsSize)) min=s;
        else max=s;
    }
    return min;
}

int main() {
    // TODO: Add the base I/O interface here
    int side, n, k;
    scanf("%d %d %d", &side, &n, &k);
    int **points = (int **)malloc(sizeof(int *) * n);
    for (int i = 0; i < n; i++) {
        points[i] = (int *)malloc(sizeof(int) * 2);
        scanf("%d %d", &points[i][0], &points[i][1]);
    }
    int pointsColSize = 2;
    printf("%d\n", maxDistance(side, points, n, &pointsColSize, k));
    free(points);
    return 0;
}