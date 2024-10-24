pub fn calc_video_time(video_len: &str, pos: &str, op_start: &str, op_end: &str, commands: Vec<&str>) -> String {
    let mut result_time= String::new();
    

    result_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = calc_video_time(
            "34:33"
            ,"13:00"
         	,"00:55"
            ,"02:55"
            , Vec::from(["next", "prev"])
        );
        assert_eq!(result, "13:00".to_string());
    }

    #[test]
    fn it_works2
    () {
        let result = calc_video_time(
            "10:55" 
            ,"00:05"
         	,"00:15"
            ,"06:55"
            , Vec::from(["prev", "next", "next"])
        );
        assert_eq!(result, "06:55".to_string());
    }

    #[test]
    fn it_works3() {
        let result = calc_video_time(
            "07:2"
            ,"04:05"
         	,"00:15"
            ,"04:07"
            , Vec::from(["next"])
        );
        assert_eq!(result, "04:17".to_string());
    } 	
}
