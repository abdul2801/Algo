// use crate::sumZero::sumZero;

// use browser_history::BrowserHistory;

// use crate::stack::MinStack;

// mod same;
// mod sumZero;
// mod countUnique;
// mod MaxSumSubArray;
// mod countodd;
// mod reverserecur;

use std::vec;

use finduniquechar::first_uniq_char;
use heap::heap;
use search_matrix::search_matri;

use crate::min_arr::find_min;

// mod ispali;
mod browser_history;
mod flatten;
mod nextgreator;
mod stack;
mod removeduplicate;
mod kdigits;
mod heap;
mod finduniquechar;
mod search_matrix;
mod eat;
mod ship;
mod days;
mod min_arr;
mod search_rot;
mod timestamp;
mod temp;
mod ispali;
mod twosumsort;
mod area;
mod stock;
mod maxsubstring;
mod minLength;
mod minpali;
mod kthlargest;
mod MakeZero;
mod zero;
mod kweakest;
mod peak;
mod min_subarr_sum;
fn main() {
  println!("{}",zero::minimum_operations(vec![1,5,0,3,5]))  
}
