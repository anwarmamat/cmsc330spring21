use crate::turtle::Turtle;
use std::rc::Rc;

/**
 * TurtleRef describes a reference to a turtle that is owned by
 * the campus. These references may need to be passed around outside
 * Campus (in this case, the test harness needs access to them).
 * We could have specified the type precisely, but that would expose
 * implementation details of Campus to the clients.
 *
 * To avoid doing that, TurtleRef serves as an intermediary whenever
 * a reference to a Turtle needs to escape Campus.
 *
 * You will need to fill in TurtleRef with a field that reflects
 * your implementation. In order to manage lifetimes correctly,
 * TurtleRef does not provide access to a &Turtle; it instead
 * supplies a BorrowedTurtle, which implements Deref.
 * 
 * Another way of thinking about this is that TurtleRef provides 
 * a persistent reference, whereas a BorrowedTurtle is ephemeral,
 * generated temporarily while the Turtle is needed.
 *
 * An example use:
 *
 * let turtle_ref: TurtleRef = campus.get_turtle(...);
 * let turtle_name: &String = turtle_ref.borrowed_turtle().name();
 *
 * This code is here because the code below it relies on it. However, we suggest
 * implementing this AFTER you have figured out how to represent Campus, not before.
 */
pub struct TurtleRef {
    // Figure out what type r should have.
    // r: ???
}

impl TurtleRef {
    pub fn borrowed_turtle(&self) -> BorrowedTurtle {
        unimplemented!();
    }

    // We suggest implementing a 'new' function.
    // It's up to you to figure out what it should take as an argument.
    // pub fn new(r: ???) -> TurtleRef {
    //     TurtleRef{r}
    // }
}

// Hint: BorrowedTurtle might need a lifetime parameter.
pub struct BorrowedTurtle {
    // Figure out what type r should have.
    // r:
}

// Hint: update this signature when you figure out the lifetime for BorrowedTurtle.
impl<'a> std::ops::Deref for BorrowedTurtle {
    type Target = Turtle;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

// Hint: update this signature when you figure out the lifetime for BorrowedTurtle.
impl<'a> std::ops::DerefMut for BorrowedTurtle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!()
    }
}

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

    pub fn turtles_with_name_cached(&self, name: &str) -> Rc<Vec<TurtleRef>> {
        unimplemented!();
    }
}
