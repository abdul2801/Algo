use std::num;

pub fn min_sub_array_len(mut target: i32, mut nums: Vec<i32>) -> i32 {
        
        let mut res=0;
        nums.sort();
        while target > 0 {
            
            let cur = nums.pop();
            if cur.is_some() {
                target-=cur.unwrap()
            }
            else {
                return 0;
            }
        
            res+=1;
            
        }
        res
}
