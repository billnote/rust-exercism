#[derive(Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    const ALLERGENS: &'static [Allergen] = &[
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    pub fn value(&self) -> u32 {
        match *self {
            Allergen::Eggs => 1 << 0,
            Allergen::Peanuts => 1 << 1,
            Allergen::Shellfish => 1 << 2,
            Allergen::Strawberries => 1 << 3,
            Allergen::Tomatoes => 1 << 4,
            Allergen::Chocolate => 1 << 5,
            Allergen::Pollen => 1 << 6,
            Allergen::Cats => 1 << 7,
        }
    }
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(s: u32) -> Self {
        Allergies { score: s }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & allergen.value()) == allergen.value()
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::ALLERGENS
            .iter()
            .filter_map(|allergen| if (allergen.value()) & self.score ==
                allergen.value()
            {
                Some(allergen.clone())
            } else {
                None
            })
            .collect()
    }
}
