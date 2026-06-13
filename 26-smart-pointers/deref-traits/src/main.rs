use std::ops::{Deref, DerefMut};

/*
 * Custom implementation of the Box Smart Pointer:
 * A Struct that behaves like a Reference despite the fact that it is not one.
 */
struct CustomBox<T, U> {
    data: T,
    more_data: U,
}

impl<T, U> CustomBox<T, U> {
    fn new(data: T, more_data: U) -> Self {
        Self {
            data,
            more_data,
        }
    }
}

impl<T, U> Deref for CustomBox<T, U> {
    //type Target = T; // -> f64
    type Target = U;

    //fn deref(&self) -> &Self::Target { // &T
    //    &self.data
    //}

    fn deref(&self) -> &Self::Target {
        &self.more_data
    }
}

impl<T, U> DerefMut for CustomBox<T, U> {
    //fn deref_mut(&mut self) -> &mut Self::Target { // &mut T
    //    &mut self.data
    //}

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more_data
    }
}

// Automatically invoked before the CustomBox goes out of scope.
impl<T, U> Drop for CustomBox<T, U> {
    fn drop(&mut self) {
        println!("I'm cleaning up related files on the hard drive");
        println!("I'm terminating a network connection");
        println!("I'm removing the CustomBox from memory");
    }
}

fn main() {
    let boxy: Box<f64> = Box::new(3.14);
    println!("{}", *boxy);

    let mut boxy: Box<f64> = Box::new(3.14);
    *boxy = 6.28;
    println!("{}", *boxy);

    //let custom_boxy: CustomBox<f64> = CustomBox::new(/*data:*/ 3.14);
    //println!("{}", *custom_boxy);
    //println!("{}", *(custom_boxy.deref())); // This is exactlty what's happening behind the scenes.
 
    let mut custom_boxy: CustomBox<f64, &'static str> = CustomBox::new(3.14, "Hello");
    //*custom_boxy = 6.28;
    *custom_boxy = "Goodbye";
    println!("{}", *custom_boxy);
}
