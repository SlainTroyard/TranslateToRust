#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// 将字符串转换为数字
int str_to_num(char str[]) {
    int num = 0;
    for (int i = 0; i < strlen(str); i++) {
        if (str[i] >= '0' && str[i] <= '9') {
            num *= 10;
            num += (int)(str[i] - '0');
        }
    }
    return num;
}

// 从id前缀中提取数字部分 (如从"id123"提取123)
int extract_id_number(const char* id_str) {
    // 查找字符串中的数字开始位置
    const char* num_start = id_str;
    while (*num_start && !(*num_start >= '0' && *num_start <= '9')) {
        num_start++;
    }
    
    // 如果没有找到数字，返回-1表示错误
    if (*num_start == '\0') {
        return -1;
    }
    
    return str_to_num(num_start);
}

int* countMentions(int numberOfUsers, char*** events, int eventsSize, int* eventsColSize, int* returnSize) {
    int time_arr[eventsSize];
    for (int i = 0; i < eventsSize; i++) {
        time_arr[i] = str_to_num(events[i][1]);
    }
    int order[eventsSize];
    for (int i = 0; i < eventsSize; i++) order[i] = i;
    for (int i = eventsSize - 1; i > 0; i--) {
        for (int j = 0; j < i; j++) {
            if (time_arr[order[j + 1]] < time_arr[order[j]] || (time_arr[order[j + 1]] == time_arr[order[j]] && events[order[j + 1]][0][0] == 'O')) {
                int t = order[j];
                order[j] = order[j + 1];
                order[j + 1] = t;
            }
        }
    }

    int online[numberOfUsers];
    int *mention = (int *)malloc(sizeof(int) * numberOfUsers);
    if (!mention) {
        // 内存分配失败处理
        *returnSize = 0;
        return NULL;
    }
    
    memset(online, 0, sizeof(online));
    memset(mention, 0, sizeof(int) * numberOfUsers);
    *returnSize = numberOfUsers;

    for (int n = 0; n < eventsSize; n++) {
        int i = order[n];
        if (events[i][0][0] == 'M') {
            if (events[i][2][0] == 'A') {
                for (int j = 0; j < numberOfUsers; j++) mention[j] += 1;
            } else if (events[i][2][0] == 'H') {
                int time = str_to_num(events[i][1]);
                for (int j = 0; j < numberOfUsers; j++) {
                    if (online[j] == 0) mention[j] += 1;
                    else if (online[j] <= time) {
                        online[j] = 0;
                        mention[j] += 1;
                    }
                }
            } else {
                char t_id[100]; // 增大缓冲区大小
                char *prev = events[i][2];
                char *space = strchr(prev, ' ');
                
                while (true) {
                    memset(t_id, '\0', sizeof(t_id));
                    if (space == NULL) {
                        // 处理最后一个ID
                        strcpy(t_id, prev);
                        int user_id = extract_id_number(t_id);
                        if (user_id >= 0 && user_id < numberOfUsers) {
                            mention[user_id] += 1;
                        }
                        break;
                    } else {
                        // 处理中间的ID
                        int len = space - prev;
                        if (len < sizeof(t_id)) {
                            strncpy(t_id, prev, len);
                            t_id[len] = '\0';
                            int user_id = extract_id_number(t_id);
                            if (user_id >= 0 && user_id < numberOfUsers) {
                                mention[user_id] += 1;
                            }
                        }
                        prev = space + 1;
                        space = strchr(prev, ' ');
                    }
                }
            }
        } else if (events[i][0][0] == 'O') {
            int user_id = extract_id_number(events[i][2]);
            if (user_id >= 0 && user_id < numberOfUsers) {
                online[user_id] = str_to_num(events[i][1]) + 60;
            }
        }
    }
    return mention;
}

int main() {
    int numberOfUsers, eventsSize;
    scanf("%d %d", &numberOfUsers, &eventsSize);
    
    char*** events = (char***)malloc(eventsSize * sizeof(char**));
    int* eventsColSize = (int*)malloc(eventsSize * sizeof(int));
    
    if (!events || !eventsColSize) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }
    
    for (int i = 0; i < eventsSize; i++) {
        events[i] = (char**)malloc(3 * sizeof(char*));
        if (!events[i]) {
            fprintf(stderr, "Memory allocation failed\n");
            return 1;
        }
        
        for (int j = 0; j < 3; j++) {
            events[i][j] = (char*)malloc(100 * sizeof(char));
            if (!events[i][j]) {
                fprintf(stderr, "Memory allocation failed\n");
                return 1;
            }
            scanf("%s", events[i][j]);
        }
        eventsColSize[i] = 3;
    }
    
    int returnSize;
    int* result = countMentions(numberOfUsers, events, eventsSize, eventsColSize, &returnSize);
    
    if (result) {
        printf("Mentions: ");
        for (int i = 0; i < returnSize; i++) {
            printf("%d ", result[i]);
        }
        printf("\n");
        
        free(result);
    } else {
        printf("Error: Failed to compute mentions\n");
    }
    
    // 释放内存
    for (int i = 0; i < eventsSize; i++) {
        for (int j = 0; j < 3; j++) {
            free(events[i][j]);
        }
        free(events[i]);
    }
    free(events);
    free(eventsColSize);
    
    return 0;
}