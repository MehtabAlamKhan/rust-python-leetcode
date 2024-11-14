#include <stdlib.h>
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
bool isBalanced(struct TreeNode *root)
{
    int helper(struct TreeNode * root, bool *isBal)
    {
        if (root == NULL)
        {
            return 0;
        }
        int left = helper(root->left, isBal);
        int right = helper(root->right, isBal);
        if (abs(left - right) > 1)
        {
            *isBal = false;
        }
        return 1 + (left > right ? left : right);
    }
    bool isBal = true;
    helper(root, &isBal);
    return isBal;
}