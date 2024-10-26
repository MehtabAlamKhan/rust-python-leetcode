class Solution:
    #time - O(m * n) 
    #space - O(n+m)
    def uniquePaths(self, m: int, n: int) -> int:
        def dp(i, j, memo):
            if (i,j) in memo:
                return memo[i,j]
            if i == m -1 and j == n - 1:
                return 1
            if i >= m or j >= n:
                return 0
            
            memo[i,j] = dp(i + 1, j, memo) + dp(i , j + 1, memo)
            return memo[i,j]
        
        return dp(0,0, {})