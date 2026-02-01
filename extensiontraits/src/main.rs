mod urgency;
mod factorial;
mod more_factorial;

// Once we import the trait, it is available
use crate::urgency::Urgency;

use crate::factorial::Factorial;

// Uncomment to see the world on fire
// use crate::more_factorial::DifferentFactorial;

fn main() {
    let mut hello:String = "Hello world".into();
    hello.add_urgency();

    let n = 10;
    
    println!("{}", hello);
    println!("The factorial of {} is {}", n, n.factorial());
}
