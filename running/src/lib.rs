`use std::ops::Index;

pub fn get_running_result(left_v: Vec<&str>, right_v: Vec<&str>) -> Option<Vec<String>> {
    let result_v: Vec<String>= Vec::new();
    let mut calling_buff: Vec<u32>= Vec::new();
    
    for e in 0..=left_v.len(){
        calling_buff.push(0);
    }
    
    let mut idx= 0;
    let mut cnt= 0;
    let right_vv= right_v.clone();

    for name_e in right_v{
        idx= right_vv.iter().position(|n| n == name_e);
        calling_buff[idx as usize]+= 1;
    }
    
    let mut laste_index_v: Vec<i32>= Vec::new();
    for e in 0..=left_v.len(){
        laste_index_v.push(0);
    }

    for idx in 0..=right_v.len(){
        laste_index_v[idx as usize]= calling_buff[idx] + idx; 
    }

    let right_vvv= right_v.clone();
    unsafe {
        let mut idx= 0_usize;
        // transaction laste_index_v -> left_v
        // laste_index_v[]
    }

    // &str -> String
    Option::from(left_v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_running_result(
            Vec::from(["mumu", "soe", "poe", "kai", "mine"])
            , Vec::from(["kai", "kai", "mine", "mine"])
        );
        let mut correct_v: Vec<String>= Vec::new();
        let unhandle_v= Vec::from(["mumu", "kai", "mine", "soe", "poe"]);
        correct_v= unhandle_v.into_iter().map(|f| f.to_string()).collect();

        assert_eq!(
            result.unwrap_or_default()
            , correct_v
        );
    }
}
