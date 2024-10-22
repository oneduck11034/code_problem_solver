pub fn clock(mut hh: i32, mut mm: i32) -> (i32, i32) {
    mm-= 45;

    if mm < 0{
        hh-= 1;
        mm= mm % 60 + 60;
    }
    if hh<0 {
        hh*= -1;
    }

    (hh, mm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = clock(10,10);
        assert_eq!(result, (9,25));
    }
}
