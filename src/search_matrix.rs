pub fn search_matri(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row = matrix.iter().find(|f| f.last().unwrap() > &target);
        match row {
            Some(r) => {
                let res =r.binary_search(&target).is_ok();
                if res {
                    true
                }
                else {
                    false
                    }

                },
                None => false,
        }
        

        


}