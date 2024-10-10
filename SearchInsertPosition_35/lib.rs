// You must write an algorithm with O(log n) runtime complexity.

struct Solution{}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if !nums.contains(&target){
           let mut idx= 0;
           for e in nums{
                if e > target{
                    return idx;
                }

                idx+=1;
           }

            idx
        }else{
            let mut idx= 0_i32;
            for e in &nums{
                if &target == e{
                    return idx;
                }
                idx+=1;
            }

            idx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums= Vec::from([1,3,5,6]);
        let target= 5_i32;

        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let nums= Vec::from([1,3,5,6]);
        let target= 2_i32;

        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works3() {
        let nums= Vec::from([1,3,5,6]);
        let target= 7_i32;

        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 4);
    }
}
