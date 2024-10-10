struct Solution{}

impl Solution {
    pub fn remove_duplicates(v: &mut Vec<i32>) -> i32 {
        let mut i=0; let mut j=0;
        let mut buff: Vec<i32>= Vec::new();
        let nested_v= v.clone();
        
        for e in v{
            let duplicated= nested_v.iter().filter(|&n| *n == *e).count();
            if buff.iter().filter(|n| n == &e).count() >= 1 {
                continue;
            }else{
                buff.push(*e);
            }
        }

        buff.len() as i32
    }
}

fn main() {
    let result= Solution::remove_duplicates(&mut Vec::from([0_i32,0,1,1,1,2,2,3,3,4]));
    assert_eq!(result, 5);
}



// 1. https://stackoverflow.com/questions/57739755/how-could-rust-multiply-i32-with-i32
// pivot 1 and 2 have a same address vector
// if pivot_1 == pivot_2{
//     buff.push(*pivot_1);
// }else{
//     continue;
// }

// 2. https://stackoverflow.com/questions/45353757/how-to-count-the-elements-in-a-vector-with-some-value-without-looping
