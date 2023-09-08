use std::ops::Div;


pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l=1;
        let mut r : i32 = *piles.iter().max().unwrap();
        while l < r {
                let mid  = l + (r-l) / 2;
                if check(mid, h, &piles) {
                    r = mid;
                }
                else {
                    l = mid +1
                }
                
            }
            l
}

pub fn check(mid : i32 , h : i32 , piles : &Vec<i32>) -> bool {
        let mut t = h;
        for i in piles {
                let cur = (i-1).div(mid) + 1;
                let c = *i;
                t-=cur;
                if t < 0 {
                   return  false;
                }
            
        }
        true
}