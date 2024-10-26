class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        len1, len2 = len(text1), len(text2)
        dp = [0] * (len2 + 1)
        for i in range(len1):
            cur = [0] * (len2 + 1)
            for j in range(len2):
                if text1[i] == text2[j]:
                    cur[j + 1] = 1 + dp[j]
                else:
                    cur[j + 1] = max(dp[j + 1], cur[j])
            dp = cur
        
        return dp[-1]
