pub fn sumZero(arr : Vec<i32>) -> (i32,i32) {
    let mut start: usize = 0;
    let mut end = arr.len()-1;
    
    while end > start {
       match arr[end]+arr[start] {

            0 => {
               
                return (start as i32,end as i32);
            }
            n if n > 0 => {
               start+=1;
            }
            _ => {
                end-=1;
            }
           
       }
    }
    (-1,-1)
}