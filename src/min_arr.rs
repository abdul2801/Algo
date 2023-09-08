
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut l=0;
    let mut r  = nums.len()-1 ;
    while l <= r {
            let mid  = l + (r-l) / 2;
          
            if nums[0] > mid as i32{
                r = mid;
            }
            else {
                l = mid +1
            }
            
        }
        nums[l]
}