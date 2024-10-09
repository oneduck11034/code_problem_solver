struct Solution{}

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(v: &mut Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        
        for e in v{
            seen.insert(e);
        }
        
        seen.len() as i32
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut inpute= Vec::from([1,1,2]);
        let result = Solution::remove_duplicates(&mut inpute);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let mut inpute= Vec::from([0,0,1,1,1,2,2,3,3,4]);
        let result = Solution::remove_duplicates(&mut inpute);
        assert_eq!(result, 5);
    }
}

// Constraints:
// 1 <= nums.length <= 3 * 104
// -100 <= nums[i] <= 100
// nums is sorted in non-decreasing order.