use std::collections::BinaryHeap;

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let num : Vec<i32> = nums.into_iter().map(|f| -f).collect();
    
    let mut q = BinaryHeap::from(num); 
    let mut res =0;
    let mut sub = 0;
    println!("{:?}",q);
    while !q.is_empty() {
        while let Some(elem) = q.peek() {
            if *elem + sub >= 0 {
                q.pop();
            } else {
                break;
            }
        }
        
        if let Some(elem) = q.peek() {
            sub -= elem;
            res += 1;
        }
    }
    res
}