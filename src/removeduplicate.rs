pub fn remove_duplicate_letters(s: String) -> String {
    let mut st: Vec<char> = Vec::new();
    
    let mut freq = [0; 26];
    let mut visit: [bool; 26] = [false ; 26];

    for x in s.chars() {
        let index = (x as u8 - 'a' as u8) as usize;
        freq[index]+=1;
    }
    for (i,x) in s.chars().into_iter().enumerate() {
        let index = (x as u8 - 'a' as u8) as usize;
        if  visit[index]{
            freq[index]-=1;
           continue; 
        }

        while st.len() != 0 && x < *st.last().unwrap() && freq[index]>0 {
            let c = st.pop().unwrap();
            let index = (c as u8 - 'a' as u8) as usize;
            visit[index] = false;
            
        }
        st.push(x);
        visit[index] = true;
        freq[index]-=1;

    }
    st.into_iter().collect()
}