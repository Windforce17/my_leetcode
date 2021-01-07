#include"acm.h"

class Solution {
public:
    vector<vector<int>> largeGroupPositions(string s) {
        vector<vector<int>> result;
        if(s.length()<3){
            return result;
        }
        auto i=0;
        auto start=0;
        auto count=1;
        char a=s[i];
        i++;
        while (i!=s.length())
        {
            if(s[i]==a){
                count++;
                if(count>=3&&i==s.length()-1){
                    result.push_back({start,i});
                    return result;
                }
            }
            else{
                if(count>=3){
                    result.push_back({start,i-1});
                }
                count=1;
                a=s[i];
                start=i;
            }
            i++;
            
        }
        return result;

    }
};