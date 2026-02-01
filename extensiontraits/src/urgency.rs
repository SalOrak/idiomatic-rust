// Lets first create the trait.
pub trait Urgency {
    fn add_urgency(&mut self);
}


// Because the trait is internal we can implement it 
// for an external struct.
impl Urgency for String {
    pub fn add_urgency(&mut self) {
        self.push('!');
    }
}
