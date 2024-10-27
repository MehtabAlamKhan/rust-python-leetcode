class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        def dp(total, i, memo):
            if (total, i) in memo:
                return memo[total, i]
            if total > amount or i >= len(coins):
                return 0
            if total == amount :
                return 1            
            memo[total, i] = dp(total + coins[i], i, memo) + dp(total, i + 1, memo)
            return memo[total, i]
        return dp(0, 0, {})