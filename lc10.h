//
// Created by zhichen.Wang on 2019-05-14.
//

#ifndef ACM_LC10_H
#define ACM_LC10_H

#include "acm.h"

//还在wa，要用dp解
class Solution {
public:
    bool isMatch(string s, string p) {
        int i, k;
        i = 0;
        k = 0;
        int p_len = (int) p.length();
        int s_len = (int) s.length();

        char p_c;
        int faild = 1;
        while ((k < p_len && i < s_len)||(k+1<p_len&&p[k+1]=='*')) {
            //不匹配 c
//            if (p[k] != s[i] && p[k] != '.')
//                if(p[k])
//                return false;
            //. .*
            if (p[k] == '.') {
                faild = 1;
                if (k + 1 < p_len && p[k + 1] == '*') {
                    k += 2;
                    if (k == p_len)return true;
                    while (i < s_len && p[k] != s[i])i++;
                    continue;
                }
                i++;
                k++;
            }
            //c* 处理0个字符的情况
            if (k + 1 < p_len && p[k + 1] == '*') {
                if (i - 1 >= 0 && s[i - 1] != p[k])faild++;
                while (s[i] == p[k])i++;
                k += 2;
                if (k - faild * 2 >= 0 && k < p_len && p[k - faild * 2] == p[k] && p[k] == s[i - 1]) {
                    i--;
                    faild = 1;
                }
            }
            //c
            if (k < p_len && i < s_len && p[k] == s[i]) {
                faild = 1;
                if (k + 1 < p_len && p[k + 1] == '*')continue;
                i++;
                k++;

            }
            //else if(k<p_len&&p[k]!='.') return false;
        }
        return k == p_len && i == s_len;

    }
};

#endif //ACM_LC10_H
