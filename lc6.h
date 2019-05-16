//
// Created by zhichen.Wang on 2019-05-13.
//

#ifndef ACM_LC6_H
#define ACM_LC6_H

#include "acm.h"
class Solution {
public:
    //模拟Z形打印..
    string convert(string s, int numRows) {
        if(!s.length())return "";
        if(numRows==1)return s;
        vector<string> result;
        result.reserve(100);
        int size = 2 * numRows - 2;
        int block = (int) s.length() / size;
        if (s.length() % size != 0)block++;
        int x = 0, y = 0;

        for (int i = 0; i < numRows; i++)
            result.emplace_back("");
        for (int i = 0; i < s.length();) {
            while (y < numRows && i < s.length())result[y++] += s[i++];
            y--;
            while (y > 0 && i < s.length())result[--y] += s[i++];
            y++;
        }
        string ret;
        for (auto s:result) {
            ret += s;
        }
        return ret;
    }
};

#endif //ACM_LC6_H
