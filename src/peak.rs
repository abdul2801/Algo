use std::num;

struct Solution;


impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l=1;
        let mut r : i32 = nums.len() as i32-2;
        if nums[0] > nums[1] {
            return 0;
        }
        else if nums[nums.len()-1] > nums[nums.len()-2] {
            return r+1;
        }
        while l < r {
                let mid: i32  = l + (r-l) / 2;
                if nums[mid as usize] >  nums[mid as usize -1] &&  nums[mid as usize ] >  nums[mid as usize +1] {
                    break
                }

                if nums[mid as usize] < nums[mid as usize -1] {
                    r = mid -1;
                }
                else {
                    l = mid +1
                }
                
            }
            l
    }
}