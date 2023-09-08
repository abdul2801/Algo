use core::time;
use std::{collections::HashMap};

struct TimeMap {
    map : HashMap<String, Vec<(i32, String)>>
   
}



impl TimeMap {

    fn new() -> Self {
        TimeMap { map: HashMap::new() }
    }
    
    fn set(& mut self, key: String, value: String, timestamp: i32) {
        let cur = (timestamp,value);
        self.map.entry(key)
            .and_modify(|t| t.push(cur.clone()))
            .or_insert_with(|| vec![cur]);
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        let t = self.map.get(&key);
        if let Some(list) = t {
            let res = list.into_iter().rev().find(|f|  f.0 <= timestamp );
        
        match res {
            Some(x) => { x.clone().1},
            None => {"".to_string()}
            
        }
        } else {
            return "".to_string();
        }
        // let mut l=0;
        // let mut r  = list.len()-1;

        // while l <= r {
        //     let mid  = l + (r-l) / 2;
        //     if list[mid].0 > timestamp {
        //         r = mid;
        //     }
        //     else {
        //         l = mid +1
        //     }

        
    //}
   
  //  (&list[l]).clone().1

        
    
    
}
}

