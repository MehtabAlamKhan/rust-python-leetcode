class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:         # type: ignore
        def dp(i, total, memo):
            if (i, total) in memo:
                return memo[i, total]
            if i == len(nums) and total == target:
                return 1
            if i == len(nums) and total != target:
                return 0             
            memo[i, total] = dp(i + 1, total + nums[i], memo) + dp(i + 1, total - nums[i], memo)
            return memo[i, total]
        return dp(0, 0, {})