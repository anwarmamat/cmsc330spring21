# Project 5: Rust Warmup

Due: April 29, 2021 (April 30th for 10% penalty, May 6th for 75% penalty)

Public tests only, no semipublic or secret tests

Ground Rules
---------------------
**This is an individual assignment. You must work on this project alone. However, you may discuss the lectures and the documentation with anyone you like.**

For this project, you are allowed to use the library functions found in `std`, including `Vec` and `String`.

You may not use any external crates for your implementation other than `image`, which is already listed as a dependency.

This assignment is about how to use Rust safely, so you may not use `unsafe`.

We are providing signatures of functions that you must implement. You must NOT change those signatures, or we will not be able to test your code. 


Introduction
------------
This project is an opportunity for you to start learning Rust.

Project Files
-------------
* Rust Files
    * __src/lib.rs__: This file implements assorted functions that you will need to complete.
    * __src/mandelbrot.rs__: This is where you will implement a generator for the Mandelbrot set.
    * __src/points.rs__: This is where you will implement functions pertaining to a Point tuple.
    * __tests/public/mod.rs__: These are the public tests. 
    * __tests/student/mod.rs__: Feel free to write your own tests here.

Compilation and Tests
---------------------
In order to compile each part of the project, simply run `cargo build` in the directory for the part of the project you're working on. To test, run `cargo test`. The tests won't run if any part of the project does not compile.

Installing Rust and Cargo
-------------------------
If you are working in grace, run `module avail rust` every time you log in.

To install Rust, please see the [Install Rust](https://www.rust-lang.org/tools/install) page.


Implementation instructions
----------------------

In __src/lib.rs__, implement these functions according to the specifications given in the comments. If you get stuck on one of them, set it aside and come back to it after you try some of the other problems.
* `gauss`
* `add_one`
* `rewrite_string`
* `to_decimal`
* `collatz`

In __src/points.rs__, implement these functions according to the specifications given in the comments:
* `rightmost_point`
* `duplicate_point`

In __src/mandelbrot.rs__, implement `make_mandelbrot_image`. We encourage you to look at the documentation of `RgbImage`. In particular, see the [code examples](https://docs.rs/image/0.23.14/image/struct.ImageBuffer.html) showing how to set pixels individually. The test will compare your output to the provided file, `mandelbrot_solution.png`, so you will need to make sure your output is exactly right. Be sure to map the number of iterations (there are between 0 and 1000 of them) to grayscale (0 to 255) linearly. All math should be done on f64 types.

Finally, back in __src/lib.rs__, implement:
* `collatz_times`
* `concatenate_strings`
* `rotate`

Resources
---------
Below we've listed some helpful links to modules you may want to consider using for your project.  The [online Rust textbook](https://doc.rust-lang.org/book/) is a terrific resource.  You can find just about anything you need for this project in the book.

* [integers][integers]
* [vectors][vectors]
* [strings][strings]
* [str][str]
* [image][image]

You are also permitted to use the Internet in a READ-ONLY way for this assignment (do not post with questions about the assignment!). Searching for the text of error messages is a good strategy if you find any of the error messages confusing.

Academic Integrity
------------------
Please **carefully read** the academic honesty section of the course syllabus. **Any evidence** of impermissible cooperation on projects, use of disallowed materials or resources, or unauthorized use of computer accounts, **will** be submitted to the Student Honor Council, which could result in an XF for the course, or suspension or expulsion from the University. Be sure you understand what you are and what you are not permitted to do in regards to academic integrity when it comes to project assignments. These policies apply to all students, and the Student Honor Council does not consider lack of knowledge of the policies to be a defense for violating them. Full information is found in the course syllabus, which you should review before starting.


[integers]: https://doc.rust-lang.org/std/primitive.i32.html
[vectors]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[strings]: https://doc.rust-lang.org/std/string/struct.String.html
[str]: https://doc.rust-lang.org/std/primitive.str.html
[derive]: https://doc.rust-lang.org/rust-by-example/trait/derive.html
[image]: https://docs.rs/image/0.23.14/image
