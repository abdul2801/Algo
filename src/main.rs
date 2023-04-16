// use crate::sumZero::sumZero;

mod same;
mod sumZero;
mod countUnique;
mod MaxSumSubArray;

fn main() {
    let res= MaxSumSubArray::MaxSumSubArray(&[1,2,3,4], 2);
    println!("{:?}",res);
 } 