class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:    # type: ignore
        res = []   
        used = [False] * len(candidates)
        def bt(target, combs, idx):
            if target == 0:
                res.append(combs.copy())
                return
            if target < 0:
                return

            for i in range(idx, len(candidates)):
                if not used[i]:
                    used[i] = True
                    c = candidates[i]
                    combs.append(c)
                    bt(target - c, combs, i)
                    used[i] = False
                    combs.pop()
        bt(target, [], 0)
        return res
                    