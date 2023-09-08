// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
// pub struct NestedIterator {
//    s : Vec<i32>
// }
// pub fn flatten(l : Vec<NestedInteger>,mut s: Vec<i32>) {
//     for x in l {
//          match x {
//              NestedInteger::Int(p) => s.push(p),
//              NestedInteger::List(k) => flatten(k,s)
//          };         
//     }
//  }


// impl NestedIterator {

//     pub fn new(nestedList: Vec<NestedInteger>) -> Self {
        
//         NestedIterator { s: Vec::new() }
//     }
    
//     pub fn next(& mut self) -> i32 {
//         self.s.pop().unwrap()
//     }
    
//     pub fn has_next(&self) -> bool {
//         self.s.len() > 0
//     }
   
// }

// // /**
// //  * Your NestedIterator object will be instantiated and called as such:

// //  */