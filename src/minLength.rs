pub fn min_length(s: String) -> i32 {
        let mut t = s.clone();
       recurse(&mut t, s.len())
}
pub fn recurse(t:&mut String , x: usize) -> i32 {
    for i in 1..x {
        if t.chars().nth(i) == Some('C') && t.chars().nth(i-1) == Some('D') || t.chars().nth(i) == Some('A') && t.chars().nth(i-1) == Some('B') {
            t.pop();
            t.pop();
            print!("{}",t);
            recurse(t, t.len());
            break;
            
        }
        
    }
    t.len() as i32

}