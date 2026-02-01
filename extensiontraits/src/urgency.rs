// Lets first create the trait.
#[allow(unused)]
pub trait Urgency {
    fn add_urgency(&mut self);
    fn add_urgency_in_spanish(&mut self);
}


// Because the trait is internal we can implement it 
// for an external struct.
impl Urgency for String {
    fn add_urgency(&mut self) {
        self.push('!');
    }

    fn add_urgency_in_spanish(&mut self) {
        self.insert(0, '¡');
        self.push('!');
    }
}

#[allow(dead_code)]
fn example() {
    let mut name: String = From::from("Hello");

    // Any string now has the method `add_urgency` as if it has always been there! 
    // Isn't that amazing?
    name.add_urgency();
    name.add_urgency();
    name.add_urgency();
    name.add_urgency();

    // But this is not the end! Let's see another example.
}


#[cfg(test)]
mod tests {
    use crate::urgency::Urgency;

    #[test]
    fn should_append_urgency() {
        {
            let mut hello = String::from("Hello World");
            hello.add_urgency();
            assert_eq!("Hello World!".to_string(), hello);
        }
        {
            let mut multiple = String::from("");
            multiple.add_urgency();
            multiple.add_urgency();
            multiple.add_urgency();
            multiple.add_urgency();
            assert_eq!("!!!!".to_string(), multiple );
        }
    }

    #[test]
    fn urgency_in_spanish(){
        {
            let mut hello = "Hola".to_string();
            hello.add_urgency_in_spanish();
            assert_eq!("¡Hola!".to_string(), hello);
        }
        {
            let mut hello = "Hola, que tal".to_string();
            hello.add_urgency_in_spanish();
            assert_eq!("¡Hola, que tal!".to_string(), hello);
        }
    }
}
