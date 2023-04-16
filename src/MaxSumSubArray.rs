

pub fn MaxSumSubArray(arr : &[i32], n : usize) -> i32 {
    // let sum : i32 = arr.iter().enumerate().filter(|(i,&_x)| i < &n).fold(0, |acc, (_, &value)| acc + value);
    let mut sum : i32 = arr.iter().take(n).sum();

    for i in n..arr.len() {
        let tempsum = sum + arr[i] - arr[i-n];
        sum = sum.max(tempsum);
    }
    sum
        
    }   
