mod person;

use crate::person::Person;

fn main() {
    let person = Person::new(String::from("Hector"), String::from("Alarcon"), 28)
        .with_job_title(String::from("Software Engineer"))
        .with_phone(12345678)
        .with_nationality(String::from("Spanish"));

    println!("Person: {:?}", person);
}
