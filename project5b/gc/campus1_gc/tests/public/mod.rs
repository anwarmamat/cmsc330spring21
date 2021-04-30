extern crate campus1_gc as campus;

use campus::campus::*;
use campus::genetics::*;
use campus::turtle::*;
use campus::cookbook::*;

static TESTUDO: &str = "Testudo";
static YERTLE: &str = "Yertle";

#[test]
fn fastest_walker_empty() {
    let campus = Campus::new();
    let t = campus.fastest_walker();
    assert!(t.is_none());
}

fn new_testudo() -> Turtle {
    Turtle::new(
        String::from(TESTUDO),
        0,
        Color::new(1, 2, 3),
        Flavor::Sweet,
    )
}

fn new_yertle() -> Turtle {
    Turtle::new(String::from(YERTLE),
                             1,
                             Color::new(4, 5, 6),
                             Flavor::Bitter)
}

#[test]
fn public_test_new() {
    let t = new_testudo();

    assert_eq!(t.name().as_str(), TESTUDO);
    assert_eq!(t.walking_speed(), 0);
    assert_eq!(*t.favorite_color(), Color::new(1, 2, 3));
    assert_eq!(t.favorite_flavor(), Flavor::Sweet);
}

#[test]
fn public_test_add_get_turtle() {
    let mut campus = Campus::new();
    let testudo = new_testudo();
    campus.add_turtle(testudo);

    let u = campus.get_turtle(0);
    assert_eq!(u.name().as_str(), TESTUDO);

    let yertle = new_yertle();
    campus.add_turtle(yertle);

    let t0 = campus.get_turtle(0);
    assert_eq!(t0.name().as_str(), TESTUDO);

    let t1 = campus.get_turtle(1);
    assert_eq!(t1.name().as_str(), YERTLE);
}

#[test]
fn public_test_size() {
    let mut campus = Campus::new();
    for i in 0..10 {
        assert_eq!(campus.size(), i);
        let testudo = new_testudo();
        campus.add_turtle(testudo);
    }
}

#[test]
fn new_testudos() {
    let mut campus = Campus::new();
    for _i in 0..10 {
        let testudo = new_testudo();
        campus.add_turtle(testudo);
    }

    let mut num_turtles = 0;
    for t in campus.turtles() {
        num_turtles = num_turtles + 1;
        assert_eq!(t.name().as_str(), TESTUDO);
    }

    assert_eq!(num_turtles, 10);
}

#[test]
fn public_test_fastest_walker() {
    let mut campus = Campus::new();

    // Check empty campus
    assert!(campus.fastest_walker().is_none());

    // Check when fastest turtle is second
    let t = new_testudo();
    campus.add_turtle(t);

    match campus.fastest_walker() {
        None => assert!(false),
        Some(f) => assert_eq!(f.name().as_str(), TESTUDO),
    }

    let yertle = new_yertle();
    campus.add_turtle(yertle);
    match campus.fastest_walker() {
        None => assert!(false),
        Some(f) => assert_eq!(f.name().as_str(), YERTLE),
    }

    // Check when fastest turtle is first
    let mut campus = Campus::new();
    // Have to make fresh Turtles because we already consumed the original ones.

    let yertle = new_yertle();
    campus.add_turtle(yertle);

    let t = new_testudo();
    campus.add_turtle(t);

    match campus.fastest_walker() {
        None => assert!(false),
        Some(f) => assert_eq!(f.name().as_str(), YERTLE),
    }
}

#[test]
fn public_test_breed_turtles() {
    let mut campus = Campus::new();
    campus.add_turtle(new_testudo());
    campus.add_turtle(new_yertle());
	
    assert_eq!(campus.size(), 2);
    campus.breed_turtles(0, 1, String::from("Mack"));
	assert_eq!(campus.size(), 3);
	if campus.size() < 3 {
	    // Abort before running remaining tests
	    return;
	}

    assert_eq!(campus.size(), 3);
    let testudo = campus.get_turtle(0);
    let yertle = campus.get_turtle(1);
    let child = campus.get_turtle(2);
    assert_eq!(*child.name(), String::from("Mack"));
    assert_eq!(child.walking_speed(), cross32(testudo.walking_speed(), yertle.walking_speed()));
	assert_eq!(*child.favorite_color(), Color::new(5, 6, 7));

    let testudo_flavor = testudo.favorite_flavor();
    let yertle_flavor = yertle.favorite_flavor();

    // If the implementation properly chooses a new flavor, the new flavor will sometimes be different from that of both parents.
    let mut found_different = false;
    for i in 0..1000 {
        campus.breed_turtles(0, 1, String::from("Mack"));
        let new_flavor = campus.get_turtle(i+3).favorite_flavor();
        if new_flavor != testudo_flavor && new_flavor != yertle_flavor {
            found_different = true;
            break;
        }
    }

    assert!(found_different);
}

#[test]
fn public_test_choose_recipe() {
    let cookbook = Cookbook::new();

    let t = new_testudo();
    let ice_cream = t.choose_recipe(&cookbook);
    assert!(ice_cream.is_some());
    assert_eq!(*ice_cream.unwrap().name(), String::from("ice cream"));

    // There's nothing bitter in the cookbook.
    let y = new_yertle();
    let none = y.choose_recipe(&cookbook);
    assert!(none.is_none());
}



#[test]
fn public_test_names() {
    let mut campus = Campus::new();
    campus.add_turtle(new_testudo());
    campus.add_turtle(new_testudo());

    let testudo_turtles = campus.turtles_with_name("Testudo");
    assert_eq!(testudo_turtles.len(), 2);

    let yertle_turtles = campus.turtles_with_name("Yertle");
    assert_eq!(yertle_turtles.len(), 0);
}
