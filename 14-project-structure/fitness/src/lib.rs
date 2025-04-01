/*
 * Custom Modules
 */
// Directory module
pub mod cardio;
// Inline module
pub mod diet {
    // Constants
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {}.", NUTRITIONIST);
    }
}
// File module
pub mod weightlifting;

// Shortcuts
use cardio::{
    self as Cardio,
    CardioTool,
    Exercise as CardioExercise,
};
use diet as Diet;
use weightlifting::{
    self as Weightlifting,
    Exercise as WeightliftingExercise,
};

// Structs
#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> Self {
        Cardio::ask_about_program();
        Diet::ask_about_program();
        Weightlifting::ask_about_program();

        Self {
            cardio,
            weightlifting,
        }
    }
}
