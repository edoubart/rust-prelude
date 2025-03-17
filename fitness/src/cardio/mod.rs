// Constants
const PERSONAL_TRAINER: &str = "Carl Cardio";

pub fn ask_about_program() {
    println!("The cardio trainer is {}.", PERSONAL_TRAINER);
}

// Enums
#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Bike,
}

// Structs
#[derive(Debug)]
pub struct Exercise {
    day: String,
    tool: CardioTool,
    minutes: u32,
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        Self {
            day,
            tool,
            minutes,
        }
    }
}
