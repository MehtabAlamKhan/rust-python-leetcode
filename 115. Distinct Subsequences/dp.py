class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        m, n = len(s), len(t)
        prev = [0] * (n + 1)
        const = [0] * (n + 1)
        const[0] = 1
        prev[0] = 1
        
        for i in range(1, m + 1):
            cur = const[:]
            for j in range(1, n + 1):
                if s[i - 1] == t[j - 1]:
                    cur[j] = prev[j - 1]
                cur[j] += prev[j]
            prev = cur
        return cur[n]

    
