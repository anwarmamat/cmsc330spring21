use bronze_gc::GcRef;

use crate::turtle::Turtle;

// A short name for a longer type. Documentation can be found here:
// https://doc.rust-lang.org/reference/items/type-aliases.html.
pub type TurtleRef = GcRef<Turtle>;

pub struct Campus {
    // Fill in some fields here.
}

impl Campus {
    pub fn new() -> Campus {
        unimplemented!();
    }

    /**
     * Returns the number of turtles present.
     */
    pub fn size(&self) -> usize {
        unimplemented!();
    }

    /**
     * This moves 'turtle', taking ownership. Do NOT implement Copy for Turtle.
     *
     * After add_turtle returns, the Campus should hold the turtle in its collection of turtles. The new turtle should be at the END of the collection.
     */
    pub fn add_turtle(&mut self, turtle: Turtle) {
        unimplemented!();
    }

    /**
     * Gets a reference to a turtle at an index. Panics if index is out of bounds.
     */
    pub fn get_turtle(&self, index: usize) -> TurtleRef {
        unimplemented!();
    }

    /**
     * Returns a fresh iterator
     */
    pub fn turtles(&self) -> std::slice::Iter<TurtleRef> {
        unimplemented!();
    }

    /**
     * Breeds the turtles at the given turtle indices, adding the new turtle to the end of the turtle vector.
     * If the indices are out of bounds, the method should panic.
     *
     * Should probably call a method you add to Turtle, which itself
     * uses the various 'cross' methods on the fields to initialize the new Turtle.
     * The new turtle should have a walking speed of 1 (babies go slowly). 
     * The new turtle should have its new favorite flavor selected randomly, 
     * and the new favorite color should be the result of crossing the 
     * parents' favorite colors.
     */
    pub fn breed_turtles(&mut self, t1_index: usize, t2_index: usize, child_name: String) {
        unimplemented!();
    }

    /**
     * Returns None if the campus is empty. Otherwise, returns Some of a reference to the turtle with the fastest walking speed.
     */
    pub fn fastest_walker(&self) -> Option<TurtleRef> {
        unimplemented!();
    }

    /**
     * Implement this for "Finding Testudo".
     * This interface will NOT work for "Fast Turtle Lookup".
     */
    pub fn turtles_with_name(&self, name: &str) -> Vec<TurtleRef> {
        unimplemented!();
    }
}
