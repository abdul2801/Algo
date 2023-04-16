// use crate::sumZero::sumZero;

mod same;
mod sumZero;
mod countUnique;

fn main() {
    let res= countUnique::count_unique(& mut [1,2,2,3]);
    println!("{:?}",res);
 }