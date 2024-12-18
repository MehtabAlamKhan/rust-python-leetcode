/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *lowestCommonAncestor(struct TreeNode *root, struct TreeNode *p, struct TreeNode *q)
{
    while (root != NULL)
    {
        if (root->val > p->val && root->val > q->val)
        {
            root = root->left;
        }
        else if (root->val < p->val && root->val < q->val)
        {
            root = root->right;
        }
        else
        {
            return root;
        }
    }
    return NULL;
}