//
// Created by zhichen.Wang on 2019-04-29.
//

#ifndef ACM_L3_H
#define ACM_L3_H

#include <string>
#include <iostream>
#include <unordered_map>
#include <algorithm>
using namespace std;

class Solution {
public:
//    int lengthOfLongestSubstring(string s) {
//        int c_s[256]={-1};
//        memset(&c_s, -1,256*4);
//        int max_len = 1;
//        if (s.length() == 0)
//            return 0;
//        int c = 0;
//        int k=0;
//        for(int i=0;i<s.length();++i){
//            if(c_s[s[i]]!=-1){
//                max_len=max(max_len,i-c);
//                for(int j=c;j<c_s[s[i]];j++){
//                    c_s[s[j]]=-1;
//                    k--;
//                }
//                c=c_s[s[i]]+1;
//                c_s[s[i]]=i;
//            } else{
//                c_s[s[i]]=i;
//                k++;
//            }
//        }
//        return max(max_len,k);
//    }
//没必要每次清空滑动后的记录了，直接记录最大值
    int lengthOfLongestSubstring(string s) {
        int c_s[256]={0};
        int max_len = 1;
        if (s.length() == 0)
            return 0;
        int c = 0;
        for(int i=0;i<s.length();++i) {
            c = max(c_s[s[i]], c);
            max_len=max(max_len,i-c+1);
            c_s[s[i]]=i+1;
        }

        return max_len;
    }
};

#endif //ACM_L3_H
