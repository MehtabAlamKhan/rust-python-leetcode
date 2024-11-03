class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        memo = {}
        nums = [1] + nums + [1]

        def dp(l, r):
            if (l,r) in memo:
                return memo[l,r]
            if l > r:
                return 0
            memo[l, r] = 0
            for i in range(l, r + 1):
                coins = nums[l - 1] * nums[i] * nums[r + 1]
                coins += dp(l, i - 1) + dp(i + 1, r)
                memo[l,r] = max(coins, memo[l,r])
            return memo[l, r]
        return dp(1, len(nums) - 2)