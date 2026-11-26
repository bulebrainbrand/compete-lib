pub trait BitControle {
    fn is_bit_one(&self, num: usize) -> bool;
    fn is_bit_zero(&self, num: usize) -> bool;
    fn get_bit_num(&self, num: usize) -> usize;
}
macro_rules! impl_int_ext {
    ($($t:ty),*) => {
        $(
            impl BitControle for $t {
                fn is_bit_one(&self,num:usize) -> bool {
                  (self & (1 << num)) != 0
                }
                fn is_bit_zero(&self,num:usize) -> bool {
                  !self.is_bit_one(num)
                }
                fn get_bit_num(&self,num:usize) -> usize {
                    if self.is_bit_one(num) {1} else {0}
                }
            }
        )*
    };
}
impl_int_ext!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_bit_one() {
        let num = 0b1101;
        assert_eq!(true, num.is_bit_one(0));
        assert_eq!(false, num.is_bit_one(1));
        assert_eq!(true, num.is_bit_one(2));
        assert_eq!(true, num.is_bit_one(3));
    }
    #[test]
    fn is_bit_zero() {
        let num = 0b1101;
        assert_eq!(false, num.is_bit_zero(0));
        assert_eq!(true, num.is_bit_zero(1));
        assert_eq!(false, num.is_bit_zero(2));
        assert_eq!(false, num.is_bit_zero(3));
    }
    #[test]
    fn get_bit_num() {
        let num = 0b1101;
        assert_eq!(1, num.get_bit_num(0));
        assert_eq!(0, num.get_bit_num(1));
        assert_eq!(1, num.get_bit_num(2));
        assert_eq!(1, num.get_bit_num(3));
    }
}
