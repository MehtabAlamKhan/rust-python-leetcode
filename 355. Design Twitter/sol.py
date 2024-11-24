class Twitter:

    def __init__(self):
        self.tweets = deque()
        self.following = {}

    def postTweet(self, userId: int, tweetId: int) -> None:
        if userId not in self.following:
            self.following[userId] = set()
        self.tweets.appendleft([tweetId, userId])        

    def getNewsFeed(self, userId: int) -> List[int]:
        res = []
        count = 0
        following = self.following.get(userId, set())
        for tid, uid in self.tweets:
            if uid in following or uid == userId:
                res.append(tid)
                count += 1
            if count >= 10:
                break
        return res

    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId in self.following:
            self.following[followerId].add(followeeId)
        else:
            self.following[followerId] = {followeeId}
        

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if self.following[followerId]:
            self.following[followerId].remove(followeeId)
        


# Your Twitter object will be instantiated and called as such:
# obj = Twitter()
# obj.postTweet(userId,tweetId)
# param_2 = obj.getNewsFeed(userId)
# obj.follow(followerId,followeeId)
# obj.unfollow(followerId,followeeId)