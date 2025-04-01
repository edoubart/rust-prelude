/***********
 * Imports *
 ***********/
use super::container::Container;

/***********
 * Structs *
 ***********/

/*
 * 'Basket' struct that can hold any kind of data.
 * Generic Struct ('T')
 */
// Attributes
pub struct Basket<T> {
    item: Option<T>,
}

// Inherent Implementations
impl<T> Basket<T> {
    // Associated Functions
    pub fn new(item: T) -> Self {
        Basket {
            item: Some(item)
        }
    }
}

// Trait Methods Implementations
impl<T> Container<T> for Basket<T> {
    // Methods

    /*
     * get(..):
     *
     * Returns the value contained by the basket wrapped in an Option. ('None'
     * if the basket had nothing.)
     */
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    /*
     * put(..):
     *
     * Stores a value replacing whatever the basket stores. *If the basket is
     * storing a number, add the new value to the existing.*
     */
    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    /*
     * is_empty(..):
     *
     * 'True' if the basket is empty.
     */
    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
