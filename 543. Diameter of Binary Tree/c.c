/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

int helper(struct TreeNode *root, int *res)
{
    if (root == NULL)
    {
        return 0;
    }
    int left = helper(root->left, res);
    int right = helper(root->right, res);
    *res = (*res > left + right) ? *res : (left + right);
    return 1 + (left > right ? left : right);
}
int diameterOfBinaryTree(struct TreeNode *root)
{
    int res = 0;
    helper(root, &res);
    return res;
}