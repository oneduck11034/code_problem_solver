struct Solution{}
impl Solution{
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let strs_v: Vec<&str> = Vec::new();        
        let bytes_v= haystack.as_bytes();
        let mut bstr= "";

        for b in bytes_v{
            let c= str::from_utf8(&b).unwarp();
            let word= bstr + c;
            strs_v.push(&word);
        }

        let flag= strs_v.into_iter().filter(|n| n.to_string() == needle).count();
        
        if flag == 0{
            return -1;
        }

        0
    }
}


fn main() {
   let result= Solution::str_str("sadbutsad".to_string(), "sad".to_string());
   let result2= Solution::str_str("leetcode".to_string(), "leeto".to_string());
   
   assert_eq!(result, 0);
   assert_eq!(result, -1);
}
