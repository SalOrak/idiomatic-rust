use std::error::Error;

use crate::urgency::Urgency;

fn main() {
    let mut name: String = From::from("Hello");

    name.add_urgency();
    name.add_urgency();
    name.add_urgency();
    name.add_urgency();

    println!("{}", name);
}
