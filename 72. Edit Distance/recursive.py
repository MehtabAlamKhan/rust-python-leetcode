class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        m, n = len(word1), len(word2)
        memo = {}
        def dp(i, j):
            if (i,j) in memo:
                return memo[i,j]
            if i == m:
                return n - j
            if j == n:
                return m - i
            
            res = 0
            if word1[i] == word2[j]:
                res = dp(i + 1, j + 1)
            else:
                res = min(dp(i, j + 1), dp(i + 1, j))
                res = min(res, dp(i + 1, j + 1))
                res = res + 1
            memo[i,j] = res
            return res
        return dp(0, 0)
            