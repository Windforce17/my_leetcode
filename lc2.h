//
// Created by zhichen.Wang on 2019-04-25.
//

#include <iostream>


struct ListNode {
    int val;
    ListNode *next;

    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
        int sum, a, b;
        auto pl1 = l1;
        auto pl2 = l2;
        while (true) {
            sum = pl1->val + pl2->val;
            a = sum / 10;
            b = sum % 10;
            pl1->val = b;
            if (pl2->next != NULL) {
                if (pl1->next == NULL) pl1->next = new ListNode(0);
                pl1->next->val += a;
                pl2 = pl2->next;
                pl1 = pl1->next;
            } else {
                if (a != 0) {
                    if (pl1->next == NULL) pl1->next = new ListNode(0);
                    pl1->next->val += a;
                }
                while (pl1->next != NULL) {
                    pl1 = pl1->next;
                    sum = pl1->val;
                    a = sum / 10;
                    b = sum % 10;
                    pl1->val = b;
                    if (a == 0) {
                        return l1;
                    }
                    if (pl1->next == NULL) {
                        pl1->next = new ListNode(0);
                    }
                    pl1->next->val += a;
                }
                break;
            }
        }
        return l1;

    }

};
