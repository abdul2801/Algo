use std::num;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l=0;
    let mut r  = nums.len()-1;

    while l < r {
        let mid  = l + (r-l) / 2;
        if nums[mid] == target {
            return mid.try_into().unwrap();
        }
        if nums[l] <= nums[mid] {
            
        
        if nums[l] <= target && target <= nums[mid] {
            r = mid 
        }
        else {
            l = mid +1

        }
    }
    else {
        if nums[r] >= target && target >= nums[mid] {
            l = mid
        }
        else {
            r = mid -1

        }
    }

    }

    -1
}