#use backtracking to get all the combinations. 
#if want only one combination then just return immediately after find the first comb

#time -
# n -> number of elements
# m -> target sum
# O(n^m)
#space = O(m^(length of combs on each stack))
class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:    # type: ignore
        res = []   
                  
        def bt(target, combs, idx):
            if target == 0:
                res.append(combs.copy())
                return
            if target < 0:
                return

            for i in range(idx, len(candidates)):
                c = candidates[i]
                combs.append(c)
                bt(target - c, combs, i)
                combs.pop()
        bt(target, [], 0)
        return res
                    