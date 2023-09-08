pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let mut counts : Vec<(usize,usize)> = mat.iter().enumerate().map(|(i,f)| {
        let c  = f.iter().filter(|x|  **x==1).count();
        return (i,c);
     }).collect();
     counts.sort_by(|a,b| b.1.cmp(&a.1));
     for i in 0..k {
          res.push(counts[i as usize].0 as i32)
         
     }
     res
}