use crate::cookbook::{Cookbook, Recipe};
use crate::genetics::*;
use crate::magic::{TurtlePower, World};

#[derive(Debug)]
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
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe(&self, cookbook: &Cookbook) -> Option<&Recipe> {
        unimplemented!();
    }
}
