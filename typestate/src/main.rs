use std::fmt::Display;
use std::{fmt};
use std::marker::PhantomData;

/// Let's imagine a Light struct. We can toggle it: light on means status is true, otherwise false.
/// This would be the normal way to do it.
struct Light {
    status:  bool,
    intensity: u8, // let's add a new attribute: intensity of the light.
}

impl Light {
    fn new() -> Self{
        Self {
            status: false,
            intensity: 0 // By default should be 0
        }
    }
    /// We just change the `status` attribute of Light.
    fn toggle(&mut self) {
        // But now, does it make sense to have a turned off ligth with 100 intensity?
        // We should change the intensity to 0. But also, if we turned it on we should have an 
        // intensity of at least 1.
        match self.is_on() {
            true => {
                // From On -> Off. The intensity should be reduced to 0.
                self.intensity = 0;
            },
            false => {
                // From Off -> On. The intensity should be at least 1. Let's say 10.
                // Which should be the minimum.
                self.intensity = 10;
            }
        }

        self.status = !self.status;
    }


    /// But now, we want to change the intensity
    fn regulate_intensity(&mut self, intensity: u8) {
        // But we should check the status and only change it there right?
        // This is starting to snowball pretty fast.
        if self.is_on() {
            self.intensity = intensity;
        }
    }

    /// let's now make a function to know if the light is on or off
    fn is_on(&self) -> bool {
        self.status
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Light is {}", {
            if self.is_on() {
                format!("On with intensity {}", self.intensity)
            } else { 
                "Off".to_string()
            }
        })
    }
}

/// Everything related to the Light implementation
fn light_one() {
    let mut light: Light = Light::new();
    println!("{}", light);
    light.toggle();
    light.regulate_intensity(200);
    println!("{}", light);
    light.toggle();
    println!("{}", light);
}

// The problem is quite clear. We have a class that is heavily dependend on certain attributes.
// Some methods don't make much sense when an attribute is a certain value. In our case, the status
// of the light changes what we CAN do. It does not make sense to change the intensity of the
// light if thus light is off! How can we solve this? Well, by making the state be reflected in the
// type itself. This is called `typestate` pattern.

/// Instead of Light we are going to call it `Luz` which is Light but in Spanish.
/// But, because we are integrating the status directly into the state machine we can do something
/// about it.
trait LuzState { }

#[derive(Debug)]
struct LuzOff {}

#[derive(Debug)]
struct LuzOn {}

impl LuzState for LuzOn { }
impl LuzState for LuzOff { }

#[derive(Debug)]
struct Luz<S: LuzState> {
    // Zero-sized type used to mark things that “act like” they own a T.
    _marker: std::marker::PhantomData<S>,
}



impl Luz<LuzOn> {
    fn toggle(self) -> Luz<LuzOff> {
       Luz {
           _marker: PhantomData
       }
    }

    fn is_on(&self) -> bool {true}
}

impl Luz<LuzOff> {
    fn new() -> Luz<LuzOff> {
        Luz {
            _marker: PhantomData
        }
    }
    fn toggle(self) -> Luz<LuzOn> {
        Luz {
            _marker: PhantomData,
        }
    }

    fn is_on(&self) -> bool {false}
}

trait LumiereState {
    fn is_on(&self)  -> bool;
}

#[derive(Debug)]
struct LumiereOff();
#[derive(Debug)]
struct LumiereOn(u8);

impl LumiereState for LumiereOn {
    fn is_on(&self)  -> bool { true }
}
impl LumiereState for LumiereOff {
    fn is_on(&self)  -> bool { false }
}

struct Lumiere<S: LumiereState> {
    state: S
}

impl<S: LumiereState> Lumiere<S> {
    fn is_on(&self) -> bool {
        self.state.is_on()
    }
}

impl Lumiere<LumiereOff> {
    fn toggle(self) -> Lumiere<LumiereOn> {
        Lumiere {
            state: LumiereOn(0)
        }
    }

    fn new() -> Lumiere<LumiereOff> {
        Lumiere::<LumiereOff> {
            state: LumiereOff()
        }
    }
}

impl Lumiere<LumiereOn> {
    fn toggle(self) -> Lumiere<LumiereOff> {
        Lumiere {
            state: LumiereOff()
        }
    }

    fn regulate_intensity(&mut self, intensity: u8) {
        self.state.0 = intensity;
    }

    fn get_intensity(&self) -> u8 {
        self.state.0 
    }
}



fn main() {
    light_one();
    let l = Luz::new();
    println!("{:?}", l);
    let l = l.toggle();
    println!("{:?}", l);
    let l = l.toggle();
    println!("{:?}", l);
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn light_impl() {
        let mut light: Light = Light::new();
        assert_eq!(light.intensity, 0);
        assert_eq!(light.is_on(), false);
        light.toggle();
        assert_eq!(light.is_on(), true);
        light.regulate_intensity(200);
        assert_eq!(light.intensity, 200);
        light.toggle();
        assert_eq!(light.is_on(), false);
    }

    #[test]
    fn luz_impl() {
        let luz = Luz::new();
        assert_eq!(luz.is_on(), false);
        let luz = luz.toggle();
        assert_eq!(luz.is_on(), true);
        let luz = luz.toggle();
        assert_eq!(luz.is_on(), false);
    }

    #[test]
    fn lumiere_impl() {
        let lumiere = Lumiere::new();
        assert_eq!(lumiere.is_on(), false);
        // assert_eq!(lumiere.get_intensity(), 0); // ERROR! It does not exist when the light is off! 
        let mut lumiere = lumiere.toggle();
        assert_eq!(lumiere.get_intensity(), 0); // Good! Now the light is on
        lumiere.regulate_intensity(220); 
        assert_eq!(lumiere.get_intensity(), 220); // Good! Now the light is on
        let lumiere = lumiere.toggle();
        assert_eq!(lumiere.is_on(), false);
    }
}
