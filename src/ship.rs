pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
      
        let mut l=1;
        let mut r : i32 = weights.iter().sum();
        
        while l < r {
            let mid  = l + (r-l) / 2;
            if check(mid, days, &weights) {
                r = mid;
            }
            else {
                l = mid +1
            }
            
        }
        r
}
pub fn check(x : i32 , days: i32 , weights: &Vec<i32> ) -> bool {
    let mut d = days;
    let mut tot = 0;
    for i in weights {
        if i < &x {
            return false;
        }
        let cur = *i;
        tot+=i;
        if tot > x {
            tot=*i;
            d-=1
            
        }
        
    }
    if d < 0 {
        return false;
    }
    true

 }
