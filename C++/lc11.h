//
// Created by zhichen.Wang on 2019-07-31.
//

#ifndef ACM_LC11_H
#define ACM_LC11_H

#include <vector>
using namespace std;
class Solution {
public:
    int maxArea(vector<int>& height) {
        int i=0,j=(int)height.size()-1 , MAXAREA=0,temp{0};
        for (int k = 0; k < height.size(); ++k) {
            if (i==j) return MAXAREA;
            temp=(j-i)*min(height[i],height[j]);
            if(temp>MAXAREA)MAXAREA=temp;
            if(height[i]<height[j])
                i++;
            else j--;
        }
        return MAXAREA;
    }
};
#endif //ACM_LC11_H
/*
 * 2 2 0 0 100 2
 * 2*6
 *  a0 a1  a2 a3
 a0
 a1 x1
 a2 x2 x1
 a3 x3 x2  x1
 */

