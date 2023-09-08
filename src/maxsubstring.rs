use std::collections::VecDeque;

pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut q: VecDeque<char>  = VecDeque::new();
        
        for e in s.chars() {

           if q.len() > res {res = q.len()}
           if  q.contains(&e) {
                while !q.is_empty() && q.pop_front().unwrap() != e  {
                    
                }

           }
           q.push_back(e);
            
        }
        res as i32
}