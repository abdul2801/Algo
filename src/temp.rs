pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut s = vec![temperatures[0]];
        for i in 0..temperatures.len() {
          
          while !s.is_empty() && temperatures[*s.last().unwrap() as usize] <= temperatures[s[i] as usize] {
                let cur = s.pop().unwrap();
                let dist =  i as i32 - cur;
                res[cur as usize] = dist;

              
          }
          s.push(i as i32);
            

        }
        res
}