/*
 * Cargo Crates
 */
use std::collections::HashSet;

/*
 * Traits
 */
pub trait Caloric {
    fn calories(&self) -> u32;
}

/*
 * Enums
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

/*
 * Structs
 */
#[derive(Debug)]
pub struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    fn new(
        protein: Protein,
        vegetables: Vec<Vegetable>,
        dressing: Dressing
    ) -> Self {
        Self {
            protein,
            vegetables,
            dressing,
        }
    }

    fn is_valid(&self) -> bool {
        self.vegetables.len() > 0
    }

    fn calories(&self) -> u32 {
        let protein_calories: u32 = self.protein.calories();

        // Option 1 (better, less iterations):
        let vegetables_calories: u32 = self.vegetables
            .iter()
            .fold(0, |mut sum, vegetable| {
                sum += vegetable.calories();
                sum
            });

        // Option 2:
        //let vegetables_calories: u32 = self.vegetables
        //    .iter()
        //    .map(|vegetable| vegetable.calories())
        //    .sum();

        let dressing_calories: u32 = self.dressing.calories();

        protein_calories + vegetables_calories + dressing_calories
    }

    // Option 1 (procedural/imperative):
    //fn has_duplicate_vegetables(&self) -> bool {
    //    let mut result: bool = false;

    //    let mut vegetables_found: HashSet<Vegetable> = HashSet::new();

    //    for vegetable in &self.vegetables {
    //        if vegetables_found.contains(&vegetable) {
    //            result = true;

    //            break;
    //        }
    //        else {
    //            vegetables_found.insert(vegetable.clone());
    //        }
    //    }

    //    result
    //}

    // Option 2 (functional/declarative):
    fn has_duplicate_vegetables(&self) -> bool {
        self.vegetables
            .clone()
            .into_iter()
            .fold(HashSet::<Vegetable>::new(), |mut data, vegetable| {
                data.insert(vegetable);
    
                data
            })
            .len()
            < self.vegetables.len()
    }
}

#[cfg(test)]
mod tests {
    /*
     * Cargo Crates
     */
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    /*
     * Custom Modules
     */
    use super::*;

    #[fixture]
    fn crispy_chicken_salad_with_two_vegetables_and_ranch_dressing() -> Salad {
        let protein: Protein = Protein::CrispyChicken;

        let tomato: Vegetable = Vegetable::Tomato;
        let cucumber: Vegetable = Vegetable::Cucumber;
        let vegetables = vec![tomato, cucumber];

        let dressing: Dressing = Dressing::Ranch;

        Salad::new(protein, vegetables, dressing)
    }

    #[rstest]
    fn salad_contains_protein_vegetables_and_dressing() {
        let protein: Protein = Protein::CrispyChicken;

        let tomato: Vegetable = Vegetable::Tomato;
        let cucumber: Vegetable = Vegetable::Cucumber;
        let vegetables = vec![tomato, cucumber];

        let dressing: Dressing = Dressing::Ranch;

        let salad: Salad = Salad::new(protein, vegetables, dressing);

        assert_eq!(salad.protein, Protein::CrispyChicken);
        assert_eq!(salad.vegetables, vec![Vegetable::Tomato, Vegetable::Cucumber]);
        assert_eq!(salad.dressing, Dressing::Ranch);
    }

    #[rstest]
    fn salad_should_have_at_least_one_vegetable(
        crispy_chicken_salad_with_two_vegetables_and_ranch_dressing: Salad
    ) {
        assert!(
            crispy_chicken_salad_with_two_vegetables_and_ranch_dressing
                .is_valid()
        );
    }

    #[rstest]
    fn salad_calculates_total_calories_from_ingredients(
        crispy_chicken_salad_with_two_vegetables_and_ranch_dressing: Salad
    ) {
        let total_calories: u32 = 400 + (20 + 15) + 150;

        assert_eq!(
            crispy_chicken_salad_with_two_vegetables_and_ranch_dressing
                .calories(),
            total_calories
        );
    }

    #[rstest]
    fn salad_can_identify_that_it_has_duplicate_vegetables() {
        let protein: Protein = Protein::CrispyChicken;

        let tomato: Vegetable = Vegetable::Tomato;
        let cucumber_1: Vegetable = Vegetable::Cucumber;
        let cucumber_2: Vegetable = Vegetable::Cucumber;
        let vegetables = vec![tomato, cucumber_1, cucumber_2];

        let dressing: Dressing = Dressing::Ranch;

        let salad: Salad = Salad::new(protein, vegetables, dressing);

        assert!(salad.has_duplicate_vegetables());
    }
}
