
pub fn ispali(s : String) -> bool {
    let s : String =s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect();
    if s.len() == 1 {true;}
    if s.len() == 2 {return s[0..1] == s[1..]}
    println!("{s}");
    if s[0..1] == s[(s.len()-1)..] {return ispali(s[1..(s.len()-1)].to_owned())}
    false
}