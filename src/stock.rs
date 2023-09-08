pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut res = 0;

    for e in prices {
        if e < min {
            min = e;
        }
        res = res.max(e-min);
        
        
    }      
    res 
}