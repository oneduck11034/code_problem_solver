use std::ops::Index;

pub fn get_running_result(left_v: Vec<&str>, right_v: Vec<&str>) -> Option<Vec<String>> {
    let result_v: Vec<String>= Vec::new();
    let mut idx= 0;
    // let calling: [String; 32] = Default::default();
    let mut calling_buff: Vec<u32>= Vec::new();
    let mut start_rank= left_v.clone();
    let orginal_rank_name= left_v.clone();

    for e_i in 0..=left_v.len() {
        calling_buff.push(0);
    }
    
    for name_e in right_v{

        if left_v.contains(&&name_e){
            
        }

    }
    
    //  names.iter().position(|n| n == name).
    idx= 0;
    for e in 0..=left_v.len() {
        start_rank.index(e) += calling_buff[e];
    }
    
    idx= 0;
    for e in left_v.len()..0 {
        // let gap= return start_rank index 
        // pointer
        // FIX result_v idx = orginal_rank_name[e + gap]
    }

    Option::from(result_v)
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
