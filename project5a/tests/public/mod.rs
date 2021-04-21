extern crate part0;

use part0::*;
use part0::mandelbrot::*;
use part0::points::*;

// 5 points
#[test]
fn public_test_gauss() {
    assert_eq!(190, gauss(19)); //prime
    assert_eq!(1, gauss(1));
    assert_eq!(54615, gauss(330));   //composite
    assert_eq!(-1, gauss(-400));
}

// 5 points
#[test]
fn public_test_add_one() {
    let mut x: i32 = 0;
    add_one(&mut x);
    assert_eq!(x, 1);
}

// 5 points
#[test]
fn public_test_concatenate_strings() {
    assert_eq!("abcd", concatenate_strings("ab", "cd"));
}

// 10 points
#[test]
fn public_test_rewrite_string() {
    let mut s = String::from("zzzz");
    rewrite_string(&mut s);
    assert_eq!(String::from("aaaa"), s);
}

// 5 points
#[test]
fn public_test_rotate() {
    let mut vec = Vec::new();
    vec.push(3);
    vec.push(3);
    vec.push(0);

    let xs = [0,3,3];
    assert_eq!(vec, rotate(&xs));

    vec.clear();

    let xs = [1];
    vec.push(1);
    assert_eq!(vec, rotate(&xs));
    vec.clear();

    let xs = [1, 1, 2, 1];
    vec.push(1); vec.push(2); vec.push(1); vec.push(1);
    assert_eq!(vec, rotate(&xs));

    let xs = [];
    vec.clear();
    assert_eq!(vec, rotate(&xs));
}

// 15 points
#[test]
fn public_test_decimal() {
    let xs = [
        false, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, true, 
        false, true, false, false, true, false, true, false];
    assert_eq!(330, to_decimal(xs));

    let ys = [
        false, false, false, false, false, false, false, false, 
    false, false, false, false, false, false, false, false, 
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false];
    assert_eq!(0, to_decimal(ys));

    let zs = [false, false, false, false, false, false, false, false, 
              false, false, false, false, false, false, false, false, 
              false, false, false, false, false, false, false, true, 
              false, true, false, true, true, true, true, true];
    assert_eq!(351, to_decimal(zs));

    let uint32_max = 
        [true, true, true, true, true, true, true, true,
         true, true, true, true, true, true, true, true,
         true, true, true, true, true, true, true, true,
         true, true, true, true, true, true, true, true];
    assert_eq!(0xffffffff, to_decimal(uint32_max));

}

// 20 points
#[test]
fn public_test_mandelbrot() {
    let image = make_mandelbrot_image();
    // Uncomment to save the image to a file so you can see your output.
    //image.save("mandelbrot.png").unwrap();
 
    let solution = image::open("tests/public/mandelbrot_solution.png").unwrap();
    assert_eq!(image, solution.to_rgb8());
}

// 10 points
#[test]
// See https://en.wikipedia.org/wiki/Collatz_conjecture for test cases
fn public_test_collatz() {
    assert_eq!(2, collatz(4));
    assert_eq!(19, collatz(9));
    assert_eq!(1132, collatz(9780657630));
}

// 5 points
#[test]
fn public_test_collatz_times() {
    let t = collatz_times(10);
    let correct = vec![0, 1, 7, 2, 5, 8, 16, 3, 19, 6];
    assert_eq!(t, correct);
}

// 10 points
#[test]
fn public_test_rightmost_point() {
    let p1 = (0.0, 42.0);
    let p2 = (1.0, 21.0);

    assert_eq!(rightmost_point(p1, p2), p2);
    assert_eq!(rightmost_point(p2, p1), p2);

    let p3 = (0.0, 17.0);
    assert_eq!(rightmost_point(p1, p3), p1);
    assert_eq!(rightmost_point(p3, p1), p3);
}

// 10 points
#[test]
fn public_test_duplicate_point() {
    let p1 = (37.0, 42.0);
    let (p1a, p1b) = duplicate_point(p1);
    assert_eq!(p1, p1a);
    assert_eq!(p1, p1b);
}