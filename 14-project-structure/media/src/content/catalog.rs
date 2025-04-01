/***********
 * Imports *
 ***********/
use super::media::Media;

/***********
 * Structs *
 ***********/

// Attributes
#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

// Inherent Implementations
impl Catalog {
    // Associated Functions
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    // Methods
    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

