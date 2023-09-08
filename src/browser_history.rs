pub struct BrowserHistory {
    b :  Vec<String>,
    f : Vec<String>
}


 impl BrowserHistory {

    pub fn new(homepage: String) -> Self {
        BrowserHistory { b: vec![homepage],f: Vec::new() }
        
    }
    
    pub fn visit(& mut self, url: String) {
        self.b.push(url);
        self.f = Vec::new();


    }
    
   pub fn back(& mut self, steps: i32) -> String {
        let mut s = steps;
        while s != 0 {
            if self.b.len() == 1 {
                break;
            }
            let cur = self.b.pop().unwrap();
            self.f.push(cur);
            s-=1;

            
        }
        return self.b.last().unwrap().to_string();


    }
    
    pub fn forward(&mut self, steps: i32) -> String {
        let mut s = steps;
        while s != 0 {
            if self.f.len() == 0 {
                break;
            }
            let cur = self.f.pop().unwrap();
            self.b.push(cur);
            s-=1;

            
        }
        return self.b.last().unwrap().to_string();
    }
}

