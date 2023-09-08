use std::vec;

pub fn heap() {
    let mut v: Vec<i64> = vec![2,3];
    for i in 1..100000000000 {
        v.push(i);
    }
    print!("{:?}",v)
}