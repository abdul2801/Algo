// pub fn remove_kdigits(num: String,mut k: i32) -> String {
//         let mut s: Vec<char> = Vec::new();

//         for (i,x) in num.chars().into_iter().enumerate() {
//             while !s.is_empty() && *s.last().unwrap() as i32 > x as i32 && k > 0 {
//                 s.pop();
//                 k-=1;
                
//             }
//             s.push(x);
            
//         }
//         while k > 0 {
//             s.pop();
//             k-=1;
//         }

        
//         if s.is_empty() {
//             return "0".to_string()
//         }

//         while *s.first().unwrap() == '0' {
//             s.remove(0);
            
//     }

//        s.into_iter().collect();
        

       

// }