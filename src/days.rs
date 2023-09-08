// pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
//     let mut l=1;
//     let mut r : i32 = *bloom_day.iter().max().unwrap();
//     while l < r {
//             let mid  = l + (r-l) / 2;
//             if check(mid, m,k, &bloom_day) {
//                 r = mid;
//             }
//             else {
//                 l = mid +1
//             }
            
//         }
//         l
// }
// pub fn check(mid: i32,m: i32,k: i32,bloom_day: &Vec<i32>)  -> bool{
//     let mut l = bloom_day.clone();
//     let res : Vec<i32> = l.iter().map(|f| {
//         if f > &mid {
//            0
//         }
//         else {
//             1
//         }
        
//     } ).collect();
//     res.iter().enumerate()
//     true
// }