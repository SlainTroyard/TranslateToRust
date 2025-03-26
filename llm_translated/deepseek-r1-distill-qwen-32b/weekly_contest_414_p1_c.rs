#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char* convertDateToBinary(char* date) {
    char * ans = (char *)calloc(25, sizeof(char));
    char * ptr = ans + 24;

    int d = atoi(date + 8);
    if (d == 0) {
        *--ptr = '0';
    } else {
        while (d) {
            *--ptr = (d & 1) + '0';
            d >>= 1;
        }
    }
    *--ptr = '-';

    int m = atoi(date + 5);
    if (m == 0) {
        *--ptr = '0';
    } else {
        while (m) {
            *--ptr = (m & 1) + '0';
            m >>= 1;
        }
    }
    *--ptr = '-';

    int y = atoi(date);
    if (y == 0) {
        *--ptr = '0';
    } else {
        while (y) {
            *--ptr = (y & 1) + '0';
            y >>= 1;
        }
    }

    size_t len = ans + 25 - ptr;
    memmove(ans, ptr, len);
    ans[len] = '\0';

    return ans;
}

int main() {
    char date[11];
    scanf("%s", date);
    char* ans = convertDateToBinary(date);
    printf("%s\n", ans);
    free(ans);
    return 0;
}