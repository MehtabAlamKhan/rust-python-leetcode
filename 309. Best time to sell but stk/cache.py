#time - O(n)
#space - O(2n)
from typing import List
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        memo = {}
        def dp(i, buying):
            if (i, buying) in memo:
                return memo[i, buying]
            if i >= len(prices):
                return 0            
            res = 0
            cooldown = dp(i + 1, buying)
            if buying:
                res = max(dp(i + 1, not buying) - prices[i], cooldown)
            else:
                res = max(cooldown, dp(i + 2, not buying) + prices[i])
            memo[i, buying] = res
            return res
        return dp(0, True)