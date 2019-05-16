//
// Created by zhichen.Wang on 2019-05-13.
//

#ifndef ACM_LC8_H
#define ACM_LC8_H

#include "acm.h"

class Solution {
public:
    int myAtoi(string str) {
        if(!str.length())return 0;
        bool nagative = false;
        int i = 0;
        long long result = 0;
        while (isblank(str[i])) i++;
        if (str[i] == '-')nagative = true;
        else if (!isnumber(str[i]) && str[i] != '+') return 0;
        if (str[i] == '+' || str[i] == '-') i++;
        for (; i < str.length(); ++i) {
            if (result > INT_MAX){
                if(nagative) return INT_MIN;
                return INT_MAX;
            }
            if (isnumber(str[i])) {
                result *= 10;
                result += (str[i] - '0');
            } else break;
        }
        if(nagative) result=0-result;
        if (result < INT_MIN) return INT_MIN;
        if (result > INT_MAX) return INT_MAX;
        return (int) result;
    }

    bool isnumber(int c) {
        return c >= '0' && c <= '9';
    }

};

#endif //ACM_LC8_H
