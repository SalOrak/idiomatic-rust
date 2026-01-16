# Typestate pattern
The typestate pattern is an idiomatic Rust pattern to enforce state
ine changes directly on types. 

For example, let's imagine we are controlling a light bulb that can be on and off. We could model it like so:
```Rust
struct Light {
    status: bool
}
impl Light {
    fn new() -> Self { Self { status: false}} // Start the light off
    fn toggle(&mut self) { self.status = ! self.status }
}
```

That is obvious, but now let's say the light has an `intensity`. Like so

```Rust
struct Light {
    status: bool,
    intensity: u8
}
impl Light {
    fn new() -> Self { Self { status: false, intensity: 0}}
    fn toggle(&mut self) { self.status = ! self.status }
    fn is_on(&self) -> { self.status}
    fn get_intensity(&self) -> u8 { self.intensity}
    fn set_intensity(&mut self, intensity: u8) -> u8 { self.intensity = intensity}
}
```

But now we have a problem. Can you see it? Yep, if we have a light on does not make much sense to have an intensity. The problem here is that now the intensity must depend on the status, if the status is off (false) then we should not be able to retrieve the intensity. It should be 0. Let's try to implement it.

```Rust
struct Light {
    status: bool,
    intensity: u8
}

impl Light {
    fn new() -> Self { Self { status: false, intensity: 0 }}
    fn toggle(&mut self) { 
        self.status = !self.status; 

        if !self.status {
            // If we end up with a Ligth off, we should reset the intensity right?
            self.intensity = 0;
        }
    }
    fn is_on(&self) -> { self.status}
    fn get_intensity(&self) -> u8 { 
        // Should we retunr the intensity even though the light is off?
        // Should we wrap the intensity in an Option then?
        self.intensity
    }
    fn set_intensity(&mut self, intensity: u8) -> u8 { 
        // What happens if I change the intensity when the Light is off?
        // Now I have a clear dependency with the status and the code gets harder.
        self.intensity = intensity
    }
}
```

Now the code has a dependecy that generates invalid code. Anyone can change the intensity even though the light is off! That should not even be a possibility. But hey, we can do it better. Let's make the intensity an `Option<u8`.
```Rust
struct Light {
    status: bool,
    intensity: Option<u8>
}

impl Light {
    fn new() -> Self { Self { status: false, intensity: None} }
    fn toggle(&mut self)  {
        self.status = ! self.status;
        if !self.status {
            self.intensity = None;
        } else {
            self.intensity = Some(0); // If the light is on we make it start with 0 intensity
        }
    }
    fn is_on(&self) -> bool { self.status }
    fn get_intensity(&self) -> Option<u8> { self.intensity }
    fn set_intensity(&mut self, intensity: u8) { 
        if self.intensity.is_some() { // Only change the intensity if there is any!
            self.intensity = Some(intensity); 
        }
    }

}
```

Okay! Problem solved right? Yes, but actually the problem is still there. The more states the Light has to go through the more complex the code will be. Right now there is only 2 different states. There are many cases which the number of states is quite big, which makes it really easy to mess up the logic. And easier to use the code wrong.

As you might have seen, we are modelling an state machine inside an struct. And we are also trying to make the API easy for clients to consume correctly. Following the example, we don't want a client accidentally changing the intesity of the Light when it is off, using that intensity and thinking the light is on when in reality, it is off. 

Lucky for us, there is an idiomatic Rust pattern that is going to solve this: the `typestate` pattern. Instead of representing the state in different variables, we represent the state directly in a type. The Rust compiler will do the rest, not allowing us to use logically incorrect code. Let's see it in action!

```Rust
// We do have 2 states. LightOn and LightOff. 
struct LightOn {}
struct LightOff {}

// The Light must accept either of them. Let's bound it by a trait
trait LightState {}
// And implement it
impl LightState for LightOn {}
impl LightState for LightOff {}

// Let's implement the struct. First without the intensity
// Light accepts is generic on S that is bounded by the LightState trait
struct<S: LightState> Light<S> { 
    // What do we put in here?  The compiler is going to complain if we do not use the generic.
    // And that is where our next friend comes into place: a PhantomData.
    // PhantomData is a zero-sized value. Just to make the compiler happy
    _marker: std::marker::PhantomData<S>
}

impl Light<LightOff> {
    fn new() -> Light<LightOff> { Light<LightOff> { _marker: PhantomData } }
    fn toggle() -> Light<LightOn> { Light<LightOn> { _marker: PhantomData } }
}

impl Light<LightOn> {
    fn toggle() -> Light<LightOn> { Light<LightOn> { _marker: PhantomData } }
}


```




