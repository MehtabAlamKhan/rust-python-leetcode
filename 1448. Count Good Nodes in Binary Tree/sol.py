# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        q = deque([[root, root.val]])
        res = 0
        while q:
            node, mx_val = q.pop()
            if node.val >= mx_val:
                res += 1
                mx_val = node.val
            if node.left:
                q.appendleft([node.left, mx_val])
            if node.right:
                q.appendleft([node.right, mx_val])
        return res
