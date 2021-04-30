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
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe(&self, cookbook: &Cookbook) -> Option<GcRef<Recipe>> {
        unimplemented!();
    }

    /**
     * This function should take ownership of the magical item and save it in a field of 'self' for future use.
     * If there is already a magical item, the new item should be saved (the old item can be discarded).
     */
    pub fn take_magical_item(&mut self, item: Box<dyn TurtlePower>) {
        unimplemented!();
    }

    /**
     * If there is a saved magical item, this function should activate it and return Ok(()).
     * Otherwise, it should return the appropriate error.
     */
    pub fn activate_magical_item(&mut self, world: &mut World) -> Result<(), NoMagicalItemError> {
        unimplemented!();
    }

    /**
     * Returns the number of children that this turtle has.
     */
    pub fn num_children(&self) -> usize {
        unimplemented!();
    }

    /**
     * Gives every child walking lessons, which increases their walking speed by 1.
     * (You think walking is easy? Try doing it while carrying your house on your back!)
     */
    pub fn teach_children(&mut self) {
        unimplemented!();
    }
}
