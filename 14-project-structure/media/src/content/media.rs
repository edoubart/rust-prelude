/*********
 * Enums *
 *********/

// Attributes
#[derive(Debug)]
pub enum Media {
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    Audiobook {
        title: String,
    },
    Podcast(u32),
    Placeholder,
}

// Inherent Implementations
impl Media {
    // Methods
    pub fn description(&self) -> String {
        //if let Media::Book { title, author } = self {
        //    format!("Book: {} {}", title, author)
        //} else if let Media::Movie { title, director } = self {
        //    format!("Movie: {} {}", title, director)
        //} else if let Media::Audiobook { title } = self {
        //    format!("Audiobook: {}", title)
        //} else {
        //    String::from("Media description")
        //}

        // Pattern Matching
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            },
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            },
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            },
            Media::Placeholder => {
                format!("Placeholder")
            },
        }
    }
}

