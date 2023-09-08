pub fn first_uniq_char(s: String) -> i32 {
        let mut q: Vec<char> = Vec::new();
        s.chars().into_iter().for_each(|f| q.push(f));

        for i in 0..s.len() {
            
            let cur = q.remove(0);
            if !q.contains(&cur) {
                return i.try_into().unwrap();
            }
            else {
                q.push(cur)
            }
            
        }
        -1

    
    }
