
#[allow(unused)]
pub trait DifferentFactorial {
    fn factorial(&self) -> Self;
}

impl DifferentFactorial for i32 {
    fn factorial(&self) -> Self {
        let mut res: Self = 1;
        for n in *self..0 {
            res *= n
        }
        res
    }
} 
