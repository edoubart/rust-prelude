/***********
 * Imports *
 ***********/
use super::container::Container;

/***********
 * Structs *
 ***********/

/*
 * 'Stack' struct that can hold as much data as needed.
 */
// Attributes
pub struct Stack<T> {
    items: Vec<T>
}

// Inherent Implementations
impl<T> Stack<T> {
    // Associated Functions
    pub fn new(items: Vec<T>) -> Self {
        Stack {
            items
        }
    }
}

// Trait Methods Implementations
impl<T> Container<T> for Stack<T> {
    // Methods

    /*
     * get(..):
     *
     * Returns the value most recently added to the Stack, 'None' if the stack
     * is empty.
     */
    fn get(&mut self) -> Option<T> {
        self.items.pop()
    }

    /*
     * put(..):
     *
     * Stores a value.
     */
    fn put(&mut self, items: T) {
        self.items.push(items);
    }

    /*
     * is_empty(..):
     *
     * 'True' if the stack is empty.
     */
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
