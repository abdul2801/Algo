// use rand::random;

use std::collections::HashMap;

pub fn same(oldarr: Vec<u32>, mut newarr: Vec<u32>) -> bool {
    for v in oldarr.iter() {
        println!("{:?}", newarr);
        if let Some(index) = newarr.iter().position(|&x| x == (v * v)) {
            newarr.remove(index);
        } else {
            return false;
        }
    }
    return true;
}

pub fn same_refactor(oldarr: Vec<u32>,  newarr: Vec<u32>) -> bool {
    let mut oldmap: HashMap<u32, u32> = HashMap::new();
    let mut newmap: HashMap<u32, u32> = HashMap::new();

    oldarr
        .iter()
        .for_each(|x| {
            oldmap.entry(*x)
                .and_modify(|t| *t += 1)
                .or_insert(1);
        });

    newarr
        .iter()
        .for_each(|x| {
            newmap.entry(*x)
                .and_modify(|t| *t += 1)
                .or_insert(1);
        });

    println!("{:?}",newmap);
    println!("{:?}",oldmap);

    for (i,v) in oldmap {
        if !(v == newmap[&(i*i)]) {
            return false;
        }
    }
    return true;

}
