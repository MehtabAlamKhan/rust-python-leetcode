class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int: # type: ignore
        rows, cols = len(matrix), len(matrix[0])
        memo = {}
        def bt(r, c, prev):
            if r < 0 or r >= rows or c < 0 or c >= cols or matrix[r][c] <= prev:
                return 0
            if (r, c) in memo:
                return memo[r,c]
            res = 0
            res = max(
                1 + bt(r + 1, c, matrix[r][c]),
                1 + bt(r - 1, c, matrix[r][c]),
                1 + bt(r, c + 1, matrix[r][c]),
                1 + bt(r, c - 1, matrix[r][c])
            )
            memo[r,c] = res
            return res
        res = 0
        for r in range(rows):
            for c in range(cols):
                res = max(res, bt(r, c, -1))
        return res

        