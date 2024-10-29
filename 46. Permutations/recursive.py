class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]: # type: ignore
        res = []
        used = [False] * len(nums)
        def bt(perms):
            if len(perms) == len(nums):
                res.append(perms.copy())
                return 
            
            for i in range(len(nums)):
                if not used[i]:
                    used[i] = True
                    perms.append(nums[i])
                    bt(perms)
                    used[i] = False
                    perms.pop()
        bt([])
        return res