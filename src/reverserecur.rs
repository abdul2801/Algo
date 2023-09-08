pub fn rev(s : String) -> String {
    if s.len() ==0 {
        return "".to_string();
    }

    let last = &s[s.len()-1..];
    let rest = s[..(s.len()-1)].to_string();
    println!("{last} {rest}");
    last.to_string() + &rev(rest)
}