class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        memo = {}
        def dp(i, j):
            if (i, j) in memo:
                return memo[i,j]
            if j == len(t):
                return 1
            if i >= len(s) or j >= len(t):
                return 0            
            res = 0
            if (s[i] == t[j]):
                res += dp(i + 1, j + 1) 
            res += dp(i + 1, j)
            
            memo[i,j] = res
            return res        
        return dp(0, 0)
            
