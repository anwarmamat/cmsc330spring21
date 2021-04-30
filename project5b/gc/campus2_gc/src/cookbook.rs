use bronze_gc::*;
use bronze_derive::*;

use crate::genetics::Flavor;

#[derive(Debug)]
pub struct Cookbook {
    recipes: Vec<GcRef<Recipe>>,
}

impl Cookbook {
    pub fn new() -> Cookbook {
        let pizza = GcRef::new(Recipe::new(String::from("pizza"), Flavor::Salty));
        let ice_cream = GcRef::new(Recipe::new(String::from("ice cream"), Flavor::Sweet));
        let mushroom = GcRef::new(Recipe::new(String::from("mushrooms"), Flavor::Umami));
        let lemon = GcRef::new(Recipe::new(String::from("lemon"), Flavor::Sour));

        Cookbook {
            recipes: vec![pizza, ice_cream, mushroom, lemon],
        }
    }

    pub fn recipes(&self) -> std::slice::Iter<GcRef<Recipe>> {
        self.recipes.iter()
    }
}

#[derive(Debug, Trace, Finalize)]
pub struct Recipe {
    name: String,
    flavor: Flavor,
}

impl Recipe {
    pub fn new(name: String, flavor: Flavor) -> Recipe {
        Recipe { name, flavor }
    }

    pub fn flavor(&self) -> Flavor {
        self.flavor
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
