//
// Created by zhichen.Wang on 2019-04-29.
//

#include "lc11.h"
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
//    vector<vector<string>> testsuits{{"bbbba",        ".*a*a"},
//                                     {"aa",          "aa"},
//                                     {"mississippi", "mis*is*p*."},
//                                     {"aab",         "c*a*b"},
//                                     {"ab",          ".*c"},
//                                     {"aa",          "ac*a"},
//                                     {"aaa",         "a*c*a"
//                                     }};
//    vector<bool> testResult{true,true, false, true, false, true, true};
//    Solution a;
//    for (int i = 0; i < testsuits.size(); i++)
//        cout << testsuits[i][0] + " " + testsuits[i][1] + ":"
//             << (testResult[i] == a.isMatch(testsuits[i][0], testsuits[i][1])) << endl;
    vector<int> tests{1,8,6,2,5,4,8,3,7};
    Solution s;
    cout<<s.maxArea(tests);
}
