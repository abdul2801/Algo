// use std::collections::{binary_heap, BinaryHeap};

// struct KthLargest {
//     s : BinaryHeap<i32>,
//     k : i32
// }


// /** 
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl KthLargest {

//     fn new(k: i32, nums: Vec<i32>) -> Self {
//         let h: BinaryHeap<i32> = BinaryHeap::new();
//         for i in nums {
//             KthLargest::add(, i);
//         }
//         KthLargest { s: h , k }
//     }
    
//     fn add(& mut self, val: i32) -> i32 {
//         self.s.push(val);

//         if self.s.len() > self.k  {
//             self.s.pop();
//         }
//         *self.s.peek().unwrap()
        
//     }
// }

