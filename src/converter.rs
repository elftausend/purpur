pub trait Converter {
    fn as_f32(&self) -> Vec<f32>;
    fn as_f64(&self) -> Vec<f64>;
    fn as_usize(&self) -> Vec<usize>;
    fn as_u8(&self) -> Vec<u8>;
}
macro_rules! converter_apply {
    ($t:ident) => {
        impl Converter for Vec<$t> {
            fn as_f32(&self) -> Vec<f32> {
                self.iter().map(|x| *x as f32).collect()
            }
            fn as_f64(&self) -> Vec<f64> {
                self.iter().map(|x| *x as f64).collect()
            }
            fn as_usize(&self) -> Vec<usize> {
                self.iter().map(|x| *x as usize).collect()
            }
            fn as_u8(&self) -> Vec<u8> {
                self.iter().map(|x| *x as u8).collect()
            }
        }
    };
}
converter_apply!(u8);
converter_apply!(u16);
converter_apply!(u32);
converter_apply!(u64);
converter_apply!(u128);
converter_apply!(usize);
converter_apply!(i8);
converter_apply!(i16);
converter_apply!(i32);
converter_apply!(i64);
converter_apply!(i128);
converter_apply!(isize);
converter_apply!(f64);
converter_apply!(f32);