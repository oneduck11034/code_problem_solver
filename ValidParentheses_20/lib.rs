pub struct Stack{
    stack: Vec<u8>,
}

impl Stack{
    fn new() -> Stack{
        let v: Vec<u8>= Vec::new();
        
        Self { stack: v }
    }
    
    fn stack_in(mut self) -> Self{
        self.stack.push(1);
        
        self
    }

    fn pop_up(mut self) -> Self{
        self.stack.pop();
        
        self
    }
}

struct Solution{}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // [40, 41 / 123, 125 / 91, 93]
        let s_bytes= s.as_bytes();
        let mut type_a: Stack= Stack::new();
        let mut type_b: Stack= Stack::new();
        let mut type_c: Stack= Stack::new();

        let a= "(".as_bytes();
        let a2= ")".as_bytes();

        let b= "{".as_bytes();
        let b2= "}".as_bytes();

        let c= "[".as_bytes();
        let c2= "]".as_bytes();

        for bc in s_bytes {
            if *bc == a[0]{
                type_a= type_a.stack_in();
            }
            if *bc == b[0]{
                type_b= type_b.stack_in();
            }
            if *bc == c[0]{
                type_c= type_c.stack_in();
            }

            if *bc == a2[0] {
                type_a= Stack::pop_up(type_a);
            }
            if *bc == b2[0] {
                type_b= Stack::pop_up(type_b);
            }
            if *bc == c2[0] {
                type_c= Stack::pop_up(type_c);
            }
        }

        let a_size= type_a.stack.len().clone();
        let b_size= type_b.stack.len().clone();
        let c_size= type_c.stack.len().clone();

        if a_size == 0 && b_size == 0 && c_size == 0 {
            true
        }else{
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_valid("()".to_string());
        assert_eq!(result, true);
    }


    #[test]
    fn it_works2() {
        let result = Solution::is_valid("()[]{}".to_string());
        assert_eq!(result, true);
    }


    #[test]
    fn it_works3() {
        let result = Solution::is_valid("(]".to_string());
        assert_eq!(result, false);
    }


    #[test]
    fn it_works4() {
        let result = Solution::is_valid("([])".to_string());
        assert_eq!(result, true);
    }
}