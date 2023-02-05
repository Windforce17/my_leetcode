//
// Created by zhichen.Wang on 2019-04-29.
//

#include "lc1744.h"
#include <iostream>
using namespace std;

//"mississippi"
//"mis*is*ip*."
struct foo
{
    int i ;
    int j ;
    foo(int x):j(x), i(j){} // i值未定义
};
int main(int argc, char **argv) {

    vector<int> a{16,38,8,41,30,31,14,45,3,2,24,23,38,30,31,17,35,4,9,42,28,18,37,18,14,46,11,13,19,3,5,39,24,48,20,29,4,19,36,11,28,49,38,16,23,24,4,22,29,35,45,38,37,40,2,37,8,41,33,8,40,27,13,4,33,5,8,14,19,35,31,8,8};
    vector<vector<int>>b{vector<int>{40,1083,86}};
    Solution s;
    
    cout<<s.canEat(a,b)[0];
}
