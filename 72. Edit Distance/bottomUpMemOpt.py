class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        m, n = len(word1), len(word2)
        dp = [0] * (m + 1)
        for i in range(m + 1):
            dp[i] = i
        for i in range(1, n + 1):
            cur = [i] + [0] * m
            for j in range(1, m + 1):
                if word1[j - 1] == word2[i - 1]:
                    cur[j] = dp[j - 1]
                else:
                    cur[j] = min(dp[j], cur[j - 1], dp[j - 1]) + 1                    
            dp = cur
        return dp[m]
        
