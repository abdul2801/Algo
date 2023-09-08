pub fn count_odd(arr : Vec<i32>) -> Vec<i32> {
    let mut newarr = Vec::new();

    if arr.len() == 0 {
        return newarr;
    }

    let (first,elem) = arr.split_first().unwrap();
    if first % 2 != 0 {
        newarr.push(*first);
    }
    newarr.append(& mut count_odd(elem.to_vec()));

    newarr

}