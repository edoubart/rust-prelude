// Shortcuts
use fitness::{
    GymWorkout,
    cardio::{
        CardioTool,
        Exercise as CardioExercise,
    },
    weightlifting::{
        Exercise as WeightliftingExercise,
    },
};

fn main() {
    let cardioExercise: CardioExercise = CardioExercise::new(
        // Day
        String::from("Monday"),
        // Tool
        CardioTool::Treadmill,
        // Minutes
        59
    );
    let weightliftingExercise: WeightliftingExercise = WeightliftingExercise::new(
        // Name
        String::from("Squat"),
        // Reps
        10
    );

    let gymWorkout: GymWorkout = GymWorkout::new(
        cardioExercise,
        weightliftingExercise
    );

    println!("Gym Workout: {:#?}", gymWorkout);
}
