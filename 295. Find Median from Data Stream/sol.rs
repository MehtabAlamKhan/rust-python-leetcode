use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct MedianFinder {
    max_h: BinaryHeap<i32>,
    min_h: BinaryHeap<Reverse<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self{
            max_h: BinaryHeap::new(),
            min_h: BinaryHeap::new()
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.max_h.is_empty() || num <= *self.max_h.peek().unwrap() {
        self.max_h.push(num);
    } else {
        self.min_h.push(Reverse(num));
    }

    // Balance the heaps to ensure the sizes differ by at most 1
    if self.max_h.len() > self.min_h.len() + 1 {
        let n = self.max_h.pop().unwrap();
        self.min_h.push(Reverse(n));
    } 
    if self.min_h.len() > self.max_h.len() {
        let n = self.min_h.pop().unwrap().0;
        self.max_h.push(n);
    }
    }
    
    fn find_median(&self) -> f64 {
        if self.max_h.len() > self.min_h.len(){
            *self.max_h.peek().unwrap() as f64
        }else if self.min_h.len() > self.max_h.len() {
            self.min_h.peek().unwrap().0 as f64
        }else{
            ((*self.max_h.peek().unwrap() + self.min_h.peek().unwrap().0)as f64) / 2.0
        }

    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */