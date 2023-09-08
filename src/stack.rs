

pub struct MinStack {
    s:Vec<i32>,
    min:i32
    
}



impl MinStack {

    pub fn new() -> Self {
        return MinStack {s : Vec::new(), min : -1};
    }
    
    pub fn push(& mut self, val: i32) {
      
        if self.s.len() == 0 {
            self.min = val;
            self.s.push(val);

        }
        else if self.s.last().unwrap() > &self.min {
            self.s.push(val);
        }
        else {
            self.s.push(2*val - self.min);
            self.min = val;
        }
        println!("{:?}",self.s);

        
    }
    
    pub fn pop(& mut self) {
        if self.s.len() == 0 {
            return
        }
        let cur = self.s.pop().unwrap();
        if cur < self.min {
            self.min = 2*self.min - cur
        }
        println!("{:?}",self.s);
    }
    
    pub fn top(&self) -> i32 {
        let cur = self.s.last().unwrap();
        if cur < &self.min {
            return 2*self.min - cur;
        }
        else {
            return *cur;
        }
    }
    
    pub fn get_min(&self) -> i32 {
        self.min
    }
}

// /**
//  * Your MinStack object will be instantiated and called as such:
//  * let obj = MinStack::new();
//  * obj.push(val);
//  * obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: i32 = obj.get_min();
//  */