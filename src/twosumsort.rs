pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for e in 0..numbers.len() {
            let cur = numbers.binary_search(&(target-numbers[e]));
            if cur.is_ok() {
                let add = if e == cur.unwrap() {1} else {0};
                res.push((e.clone()+1) as i32);
                res.push((cur.unwrap()+add) as i32);
                break;
            }
        };

        res
}