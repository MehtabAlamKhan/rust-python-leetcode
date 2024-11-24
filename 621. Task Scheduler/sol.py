class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        counts = {}

        for char in tasks:
            counts[char] = 1 + counts.get(char, 0)

        maxHeap = [-i for i in counts.values()]
        heapq.heapify(maxHeap)

        time = 0
        q = deque()

        while maxHeap or q:
            time += 1
            if maxHeap:
                t = heapq.heappop(maxHeap)
                if t + 1 != 0: q.append([t + 1, time + n])
            if q and time == q[0][1]:
                heapq.heappush(maxHeap, q.popleft()[0])
        
        return time
        

        