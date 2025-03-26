// Translated from CPP to Rust using LLM
// Original: Weekly Contest 434 Problem 2

// Note: This is a placeholder translation because no LLM API is configured or call timed out.
// To use actual LLM translation, set up API credentials in environment variables or config file.

use std::io;

struct Solution;

impl Solution {
    // Placeholder for C++ method: bool cmp(vector<string>& a, vector<string>& b)
    fn cmp(a: &str, b: &str) -> bool {
        // Placeholder implementation
        false
    }

    // Placeholder for C++ method: int extractId(const string& idStr)
    fn extractId(idStr: &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: 开头 if(idStr.substr(0, 2)
    fn if() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: return stoi(idStr.substr(2)
    fn stoi() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: return stoi(idStr)
    fn stoi() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: void updateCounts(vector<int>& counts, string& mentions)
    fn updateCounts(counts: &str, mentions: &str) -> () {
        // Placeholder implementation
        ()
    }

    // Placeholder for C++ method: istringstream iss(mentions)
    fn iss() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: 用空格分割字符串 while(iss >> idStr)
    fn while(idStr: &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: else if(mentions == "HERE")
    fn if("HERE": &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: else updateCounts(counts, mentions)
    fn updateCounts() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: int main()
    fn main() -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: 可能包含空格 if(events[i][0] == "MESSAGE")
    fn if("MESSAGE": &str) -> i32 {
        // Placeholder implementation
        0
    }

    // Placeholder for C++ method: else if(events[i][0] == "OFFLINE")
    fn if("OFFLINE": &str) -> i32 {
        // Placeholder implementation
        0
    }

}

fn main() {
    // This is a placeholder implementation
    // In a real scenario, the LLM would translate the C++ I/O to Rust
    
    println!("Placeholder implementation. To get a proper translation, configure LLM API.");
}

/*
Original CPP code:
// Problem: Weekly Contest 434 Problem 2
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <regex>
#include <sstream>

using namespace std;

class Solution {
private:
    static bool cmp(vector<string>& a, vector<string>& b){
        int time_a = stoi(a[1]);
        int time_b = stoi(b[1]);
        return (time_a == time_b && a[0] != b[0]) ? a[0] == "OFFLINE" : time_a < time_b;
    }

    // 提取ID中的数字部分
    int extractId(const string& idStr) {
        // 检查是否以"id"开头
        if (idStr.substr(0, 2) == "id") {
            try {
                return stoi(idStr.substr(2));
            } catch (const std::exception& e) {
                cerr << "Error parsing ID: " << idStr << endl;
                return 0; // 返回默认值
            }
        } else {
            // 尝试直接转换整个字符串
            try {
                return stoi(idStr);
            } catch (const std::exception& e) {
                cerr << "Error parsing ID: " << idStr << endl;
                return 0; // 返回默认值
            }
        }
    }

    void updateCounts(vector<int>& counts, string& mentions){
        istringstream iss(mentions);
        string idStr;
        
        // 用空格分割字符串
        while (iss >> idStr) {
            try {
                int id = extractId(idStr);
                if (id >= 0 && id < counts.size()) {
                    counts[id]++;
                }
            } catch (const std::exception& e) {
                cerr << "Error in updateCounts: " << e.what() << endl;
            }
        }
    }

public:
    vector<int> countMentions(int numberOfUsers, vector<vector<string>>& events) {
        sort(events.begin(), events.end(), cmp);
        vector<int> onlineTimes(numberOfUsers, 0);
        vector<int> counts(numberOfUsers, 0);
        int allCount = 0;
        
        for(vector<string>& event: events)
        {
            int time = stoi(event[1]);
            if(event[0] == "OFFLINE")
            {
                try {
                    int id = extractId(event[2]);
                    if (id >= 0 && id < numberOfUsers) {
                        onlineTimes[id] = time + 60;
                    }
                } catch (const std::exception& e) {
                    cerr << "Error processing OFFLINE event: " << e.what() << endl;
                }
                continue;
            }
            
            string mentions = event[2];
            if(mentions == "ALL")
                allCount++;
            else if(mentions == "HERE")
            {
                for(int i=0; i<numberOfUsers; i++)
                    if(onlineTimes[i] <= time)
                        counts[i]++;
            }
            else
                updateCounts(counts, mentions);
        }
        
        if(allCount > 0)
            for(int& count: counts)
                count += allCount;
        
        return counts;
    }
};

int main() {
    int numberOfUsers, numberOfEvents;
    cin >> numberOfUsers >> numberOfEvents;
    
    vector<vector<string>> events(numberOfEvents, vector<string>(3));
    for (int i = 0; i < numberOfEvents; i++) {
        cin >> events[i][0] >> events[i][1];
        
        // 单独处理第三个参数，可能包含空格
        if (events[i][0] == "MESSAGE") {
            string messageType;
            cin >> messageType;
            
            if (messageType == "ALL" || messageType == "HERE") {
                events[i][2] = messageType;
            } else {
                // 这是提及用户的情况，需要读取整行
                events[i][2] = messageType;
                string restOfLine;
                getline(cin, restOfLine);
                if (!restOfLine.empty()) {
                    events[i][2] += restOfLine;
                }
            }
        } else if (events[i][0] == "OFFLINE") {
            cin >> events[i][2];
        }
    }
    
    try {
        Solution solution;
        vector<int> result = solution.countMentions(numberOfUsers, events);
        
        for (int count : result) {
            cout << count << " ";
        }
        cout << endl;
    } catch (const std::exception& e) {
        cerr << "Exception in main: " << e.what() << endl;
        return 1;
    }
    
    return 0;
}

*/
