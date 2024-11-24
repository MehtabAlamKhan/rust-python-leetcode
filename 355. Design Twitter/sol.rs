use std::collections::{HashMap, HashSet, VecDeque};
struct Twitter {
    following: HashMap<i32, HashSet<i32>>, 
    tweets: VecDeque<(i32, i32)> //[[tweetId, userId]]
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self{
            following: HashMap::new(),
            tweets: VecDeque::new()
        }        
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push_front((tweet_id, user_id));
        self.following.entry(user_id).or_insert(HashSet::new());
    }
    
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut count = 0;
        let mut following = self.following.entry(user_id).or_insert(HashSet::new());
        for (tid, uid) in &self.tweets{
            if following.contains(&uid) || *uid == user_id{
                count += 1;
                res.push(*tid);
            }
            if count >= 10{
                break;
            }
        }
        res        
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.following.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.following.get_mut(&follower_id).unwrap().remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */