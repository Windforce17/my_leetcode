//蛇形数组
#include <cstdio>
#include <cstring>
#include <vector>
#include <iostream>
#include <string>
#include <sstream>
#include <set>

using namespace std;
#define MAXN 20
int a[MAXN][MAXN];
set<string> dict;

int snakeArray() {
    int n, tos, x, y;
    scanf("%d", &n);
    tos = 1;
//    x = -1;
//    y = n-1;
    x = -1;
    y = n - 1;
    memset(a, 0, sizeof(a));
//    a[x][y] = tos;
    while (tos < n * n) {
//        for (; x < n-1 && a[x+1][y] == 0; x++) {
//            a[x+1][y] = tos++;
//        }
//        for (; y > 0 && a[x][y-1] == 0; --y) {
//            a[x][y-1] = tos++;
//        }
//        for(;x>0&&a[x-1][y]==0;x--){
//            a[x-1][y] = tos++;
//        }
//        for (; y <n-1&& a[x][y+1] == 0 ; y++) {
//            a[x][y+1] = tos++;
//
//        }
        while (!a[x + 1][y] && x < n - 1) {
            a[++x][y] = ++tos;
        }
        while (!a[x][y - 1] && y > 0) {
            a[x][--y] = ++tos;
        }
        while (!a[x - 1][y] && x > 0) {
            a[--x][y] = ++tos;
        }
        while (!a[x][y + 1] && y < n - 1) {
            a[x][++y] = ++tos;
        }
    }
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            printf("%d\t ", a[i][j]);
        }
        printf("\n");
    }
}
