# Project 5b: Turtles for sale!

Due: May 11, 2021

Public tests only, no semipublic or secret tests

Ground Rules
---------------------
**This is an individual assignment. You must work on this project alone. Unlike in other assignments, in which you might have high-level discussions with others, you must not discuss the project (even at a high level) with others (except course staff). However, you may discuss the lectures and the documentation with anyone you like.**

For this project you are allowed to use the library functions found in `std`, including `Vec`, `String`, `collections::HashMap`, and `Box`.

You are encouraged to use `use` appropriately to facilitate referencing types from other files conveniently.

You may not use any external crates for your implementation.

This assignment is about how to use Rust safely, so you may not use `unsafe`.

We are providing signatures of functions that you must implement. You must NOT change those signatures, or we will not be able to test your code. HOWEVER, you MAY add or change lifetime parameters in structs, impls, and functions as needed. You will need to do so in some cases in order to complete this assignment successfully.

The Study
--------------
We are conducting a study in this course to better understand how people learn and use Rust. In particular, we are interested in how the optional use of garbage collection affects programmers' experience using Rust. Please see our [recruitment letter](http://www.cs.umd.edu/~mcoblenz/CMSC330/study-recruitment.pdf).

The study consists of two optional parts:

1. *The experiment*: if you agree to participate in the experiment, we will randomly assign to *either* use garbage collection or not. If you do *not* participate in the experiment, you may choose which version of the assignment to do. 

2. *Surveys*: We would like you to complete questionnaires telling us about your experience with each part of the project. There are four short questionnaires. At the end of each part of the project, complete one survey: https://umdsurvey.umd.edu/jfe/form/SV_9mLFzBb4EscoDJ4

Extra Credit:

* If you do the version of the assignment that we randomly assign, you will receive **5%** extra credit on P5. For example: If you participate in the experiment and complete the survey, and your P5 grade is 90, you will receive 4.5 points extra credit for Project 5.
* If you sign up to do the experiment but withdraw (and do the other version of the assignment instead), you will receive **2.5%** extra credit.

* If you complete the surveys after doing each part of the assignment, you will receive **1%** extra credit on project 5 for **each** survey you complete. However, if you do all of your assigned surveys, you will receive **5%** extra credit. The extra credit will *not* depend on what you write in the questionnaires, but we trust you to do them completely and honestly.

To sign up for one or both parts of the study, use this link: https://umdsurvey.umd.edu/jfe/form/SV_afaGZFxfZtw8Dv8. That page will automatically assign you to a version of the assignment (if you agree to be randomly assigned). Regardless, it will send you links to the surveys.

Using garbage collection for the project may help you learn Rust faster, allowing you to wait until the final part of the project to learn some details of how to manage references in Rust. If you use garbage collection for the project, you will complete an additional part of the project at the end, in which you remove your use of garbage collection, so you will learn the same amount of Rust as you would if you do NOT use garbage collection. However, you may find the earlier parts of the assignment easier, which may make up for the additional part.

If you do NOT use garbage collection, you will instead be using traditional techniques from the Rust standard library to manage memory.

If you are NOT an experiment participant and you change your mind about which version you want to complete you want to take, you will need to complete all of the programming tasks for the version you select in the end. If you ARE an experiment participant, you may withdraw from the experiment at any time in order to change which approach you are using. Simply submit the withdrawal form (use the link you will receive by email when you sign up for the study) and then complete the appropriate programming tasks. Do NOT try out the other approach without withdrawing from the experiment.

If you participate in the surveys, please keep track of the amount of time you spend on each part of this project for research purposes. Your grade will not depend on how long you spend on the project. Free tools for doing this include Clockify (https://clockify.me) and Toggl Track (https://toggl.com/track). We will not ask you for any details (e.g. at what times of day you worked); we will only ask for total number of hours. In addition to time spent editing, testing, and debugging code, you should include any time you spend on project-related tasks, such as reviewing slides, attending office hours to learn about Rust or the project, 

Introduction
------------
You may have heard of the *Nonfungible Token* (NFT) craze. Or you may have heard of CryptoKitties (https://www.cryptokitties.co). In this project, you will be filling campus with cool turtles! Every turtle is unique, and pairs of turtles can breed to make new turtles. We hope that creating your own NFT infrastructure will move you further down the path to fame and fortune!

You should complete Part 1 with the files in `campus1` (or `campus1_gc` if you are using garbage collection). Then, complete Part 2 with the files from `campus2` (or `campus2_gc`). You may find it useful to re-use your solutions from part 1 in part 2, but note that some of the signatures change between parts. If you are using garbage collection, after you complete `campus2_gc`, you should also complete the tasks in `campus2`. You may re-use your code from `campus2_gc` as you see fit.

Perhaps you are already used to working this way, but we *strongly recommend* writing small amounts of code at a time, and then making sure your new code compiles and runs correctly. If you try to do all of the implementation work and only compile afterward, you are likely to find yourself mired in error messages. You may want to `git commit` locally before you make major changes so that you can back up and try a different approach if needed.

Project Files
-------------
* Rust Files
    * __src/campus.rs__: This file implements the campus, where the turtles live.
    * __src/cookbook.rs__: This represents a cookbook, which the turtles use to select recipes.
    * __src/genetics.rs__: This implements traits that the turtles have and supports combining traits when breeding turtles.
    * __src/magic.rs__: This file implements turtle superpowers.
    * __src/turtle.rs__: This file implements the `Turtle` struct.
    * __tests/public/mod.rs__: These are the public tests. 
    * __tests/student/mod.rs__: Feel free to write your own tests here.

Compilation and Tests
---------------------
In order to compile each part of the project, simply run `cargo build` in the directory for the part of the project you're working on. To test, run `cargo test`. The tests won't run if any part of the project does not compile.

Installing Rust and Cargo
-------------------------
### Working on GRACE
* If you are NOT using garbage collection: run `module avail rust` every time you log in.

* If you ARE using garbage collection, you need to install `rustup` locally.
  
   * Run `curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2021-03-22`.
   You will get a complaint indicating that `rustc` is already installed. Install anyway (type `y`).
   * Installation will take a few minutes, but you only have to do this once.
   * Then, update your path: `echo 'set path = ( $HOME/.cargo/bin $path )' >> ~/.path`


### Working locally
You should have already installed Rust for the previous assignment. If you need instructions on installing Rust, please see the [Install Rust](https://www.rust-lang.org/tools/install) page.

If you are using GC, you need to be on the "nightly" version of Rust. We have tested with the March 22, 2021 version, so we recommend you use that.  
* To install, run `rustup install nightly-2021-03-22`.

* Run `rustup default nightly-2021-03-22` to set that as your default version of rustc.


If you are using garbage collection
------------------------------------
Complete parts 1 and 2 using the GC starter code (`gc/campus1_gc` and `gc/campus2_gc`). Then, complete the tasks in `gc/campus2`, in which you will adapt the code you have already written to no longer use garbage collection.

If you are not using garbage collection
----------------------------------------
Complete parts 1 and 2 using the "non-GC" starter code (`traditional_rust/campus1` and `traditional_rust/campus2`).

Part 1: Independent turtles
----------------------
In this part of the project, you'll be setting up some basic infrastructure needed for tracking turtles on campus. Turtles each have individual properties, such as walking speed and favorite flavor. You'll need to implement these to make each turtle special You'll also need to implement code related to breeding turtles so we can populate campus.

### Basics

We recommend that within Part 1, you choose to do these tasks in an order that makes sense to you, not necessarily in the order listed here. In particular, it is useful to add functionality in an order that will allow you to test your code as you write it (rather than at the very end). You should read the whole list of tasks, make a strategy, and then get started.

Implement `cross` on the `Color` `impl`. You must invoke the appropriate function defined in `genetics.rs`.

In turtle.rs, implement:
* `new` function according to the given signature. 
* Accessors `walking_speed`, `favorite_flavor`, `favorite_color`, and `name`. 

Campus should maintain a vector  (`Vec`) of turtles. In campus.rs, implement methods:
* `new`: creates a new, empty Campus
* `size`: returns the number of turtles on campus
* `add_turtle`: adds a new Turtle to campus.
* `get_turtle`: returns a reference to a turtle at a given index.
* `turtles`: returns an iterator that a caller can use to iterate through the turtles.
* `fastest_walker`: Returns None if the campus is empty. Otherwise, returns Some of a reference to the turtle with the fastest walking speed.
* `breed_turtles`, which uses the functions in `genetics` to breed two turtles, resulting in a new Turtle. You will want to add a new `breed` function to Turtle to avoid exposing Turtle implementation details. The new turtle should have its new favorite flavor selected randomly, and the new favorite color should be the result of crossing the parents' favorite colors. The new turtle's walking speed should be 1 (babies go slowly).

You might be surprised to learn that turtles love to cook. Given a cookbook, a turtle can choose a favorite recipe according to its own tastes. Implement `choose_recipe` in turtle.rs. You will need to add lifetime parameters, but do not make any other changes to the signature.

### Finding Testudo

Every turtle has a name. Of course, as with people, several turtles may have the same name. In campus.rs, implement `turtles_with_name` so that it returns a vector of turtles that have the given name. 

After you have completed this and run your own tests, you should submit part 1 to Gradescope.

Part 2: Turtle Organization
----------------------
### Getting started
Part 2 is in the `campus2` or `campus2_gc` directory, depending on which version of the assignment you're doing.

Many of the functions in part 2 are the same as those in part 1. You should start by copy/pasting the code you already wrote, but note that some of the signatures changed and you will need to make minor changes as a result.

### Turtle magic
In turtle.rs, implement methods:
* `take_magical_item`
* `activate_magical_item`

When testing, we will replace magic.rs with our own version. Do not rely on any of the behavior that is currently implemented there. `take_magical_item` needs to support storing any trait object that implements `TurtlePower`, and `activate_magical_item` should call `activate`, resulting in the behavior implemented in the appropriate `activate` method.


### Children
It can be helpful for parents to keep track of their children. Unfortunately, in part 1, parents don't know who their children are! Let's fix that. Change `Turtle` so that each turtle has a field that keeps track of its children in a `Vec`. `Vec` of what? You'll have to figure that out. Do NOT store indices into the Campus's vector because those may change in the future (some day we may support removing turtles from Campus). Do NOT invent your own indexing scheme and store a map somewhere.

If you breed two turtles, each parent should include the child in its list of children. In turtle.rs, implement methods:
* `num_children`
* `teach_children`

For example, if there are no children yet, and you breed turtles 0 and 1 to make turtle 2, then num_children() should now yield 1 on each of turtles 0 and 1.

Note that this task may require you to revisit some of the decisions you made in Part 1. You are welcome to copy/paste implementations from your Part 1 work, but note that some of the signatures are different (in order to facilitate the design changes you will need to make).

### Performance optimization
Your previous implementation of `turtles_with_name` had to do a linear search through the whole vector of turtles. If the campus contains many turtles, this can be very slow. Improve the performance of `turtles_with_name` by adding a cache to Campus. Your cache should be a HashMap that maps from turtle names (strings) to...well, you'll figure it out. Do not modify your original implementation of `turtles_with_name`; instead, implement `turtles_with_name_cached`.

After you have completed this and run your own tests, you should submit project5b to Gradescope.

Part 3: No more garbage collection
-----------------------------------
This part only applies if you are using garbage collection. Complete the tasks from part 2 above using `campus2` rather than `campus2_gc`. You may re-use code from `campus2_gc` as you see fit.  Note that you should be working in the `gc/campus2` directory for this part.

Resources
---------
Below we've listed some helpful links to modules you may want to consider using for your project.  The [online Rust textbook](https://doc.rust-lang.org/book/) is a terrific resource.  You can find just about anything you need for this project in the book. If you are using garbage collection, there is [documention available](https://crates.io/crates/bronze_gc). You will not need `GcNullableRef` in this project.

* [integers][integers]
* [vectors][vectors]
* [strings][strings]
* [str][str]
* [iterators][iterators]: some notable functions being collect, enumerate, zip 
* [options][options]
* [derive][derive]

You are also permitted to use the Internet in a READ-ONLY way for this assignment (do not post with questions about the assignment!). Searching for the text of error messages is a good strategy if you find any of the error messages confusing.

Academic Integrity
------------------
Please **carefully read** the academic honesty section of the course syllabus. **Any evidence** of impermissible cooperation on projects, use of disallowed materials or resources, or unauthorized use of computer accounts, **will** be submitted to the Student Honor Council, which could result in an XF for the course, or suspension or expulsion from the University. Be sure you understand what you are and what you are not permitted to do in regards to academic integrity when it comes to project assignments. These policies apply to all students, and the Student Honor Council does not consider lack of knowledge of the policies to be a defense for violating them. Full information is found in the course syllabus, which you should review before starting.


[integers]: https://doc.rust-lang.org/std/primitive.i32.html
[vectors]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[strings]: https://doc.rust-lang.org/std/string/struct.String.html
[str]: https://doc.rust-lang.org/std/primitive.str.html
[iterators]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[options]: https://doc.rust-lang.org/std/option/enum.Option.html
[derive]: https://doc.rust-lang.org/rust-by-example/trait/derive.html
