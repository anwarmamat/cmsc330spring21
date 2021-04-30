use bronze_gc::*;
use bronze_derive::*;

use crate::cookbook::{Cookbook, Recipe};
use crate::genetics::*;
use crate::magic::{TurtlePower, World};

#[derive(Trace, Finalize, Debug)]
pub struct Turtle {
    name: String,
    walking_speed: u32,
    favorite_color: Color,
    favorite_flavor: Flavor,
}

pub struct NoMagicalItemError;

impl Turtle {
    /**
     * Returns an appropriately-initialized Turtle.
     */
    pub fn new(name: String, walking_speed: u32, favorite_color: Color, favorite_flavor: Flavor) -> Turtle {
        unimplemented!();
    }

    pub fn walking_speed(&self) -> u32 {
        unimplemented!();
    }

    pub fn favorite_flavor(&self) -> Flavor {
        unimplemented!();
    }

    pub fn name(&self) -> &String {
        unimplemented!();
    }

    pub fn favorite_color(&self) -> &Color {
        unimplemented!();
    }

    /**
     * Returns Some of any recipe from the given cookbook that matches the turtle's flavor
     * preferences, or None if no such recipe exists.
     */
    pub fn choose_recipe(&self, cookbook: &Cookbook) -> Option<GcRef<Recipe>> {
        unimplemented!();
    }
}
