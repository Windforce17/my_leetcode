//
// Created by zhichen.Wang on 2019-04-29.
//

#include "lc10.h"

using namespace std;

//"mississippi"
//"mis*is*ip*."

int main(int argc, char **argv) {
    vector<vector<string>> testsuits{{"bbbba",        ".*a*a"},
                                     {"aa",          "aa"},
                                     {"mississippi", "mis*is*p*."},
                                     {"aab",         "c*a*b"},
                                     {"ab",          ".*c"},
                                     {"aa",          "ac*a"},
                                     {"aaa",         "a*c*a"
                                     }};
    vector<bool> testResult{true,true, false, true, false, true, true};
    Solution a;
    for (int i = 0; i < testsuits.size(); i++)
        cout << testsuits[i][0] + " " + testsuits[i][1] + ":"
             << (testResult[i] == a.isMatch(testsuits[i][0], testsuits[i][1])) << endl;
}
