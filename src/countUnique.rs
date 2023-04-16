

pub fn count_unique(arr : &mut [i32]) -> i32 {
    
    let mut ptr1 = 0;
    for i in 0..arr.len() {
        if arr[i] != arr[ptr1] {
            ptr1+=1;
            arr[ptr1] = arr[i]
        }
    }

    ptr1 as i32 +1
}