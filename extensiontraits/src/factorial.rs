// Programming is just moving data around, changing it from one format to another.
// That make sound controversial, and it is. 
// But there is proof that works. 
// This is called Data oriented programming.
// Check out the following videos:
// [Andrew Kelly: Applying Data Oriented Design](https://www.youtube.com/watch?v=IroPQ150F6c)
// [Mike Acton: Data Oriented Design](https://www.youtube.com/watch?v=rX0ItVEVjHc)


// Extension Trait might not feel related to Data Oriented Design. But I disagree. 
// Extension traits allows any developer to extend the functionality of current data type, giving
// them new life.

// And here let's see how we can implement the factorial function to numbers like they've always
// had it. Duh!

// First, let's define the trait
pub trait Factorial {
    fn factorial(&self) -> Self;
}

impl Factorial for i32 {
    // The stupidest factorial ever.
    fn factorial(&self) -> Self {
        let mut res: Self = 1;
        // We need to deref it manually to become a number
        for n in 1..((*self) + 1) {
            res *= n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    // Once it is imported, everyone can use it!
    use crate::factorial::Factorial;

    #[test]
    fn does_it_work_i32() {
        {
            let n: i32 = 1;
            assert_eq!(1, n.factorial());
        }
        {
            let n: i32 = 2;
            assert_eq!(2, n.factorial());
        }
        {
            let n: i32 = 3;
            assert_eq!(6, n.factorial());
        }
        {
            let n: i32 = 10;
            assert_eq!(3_628_800, n.factorial());
        }
    }
}

