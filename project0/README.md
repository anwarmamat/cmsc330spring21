# Project 0: Setup
Due: 10 Feb 2021 at 23:59 EST

This project is simply to get your system ready.  You will "submit" this project for a grade.  The good-faith attempt (GFA) rule **does not apply** to this project.

## Languages and Packages

In this course, we will primarily be using Ruby and OCaml.  Below is a summary of the packages that need to be installed.  You do not need to use these links, they are just for reference or learning more about the languages and/or packages.  You can skip below to the instructions.

- [Git](https://git-scm.com/)
- [Ruby](https://www.ruby-lang.org)
    - [minitest](https://rubygems.org/gems/minitest)
    - [sqlite3](https://rubygems.org/gems/sqlite3)
    - [sinatra](https://rubygems.org/gems/sinatra)
- [OCaml](http://ocaml.org)
    - [OPAM](https://opam.ocaml.org)
    - [OUnit](https://opam.ocaml.org/packages/ounit)
    - [dune](https://opam.ocaml.org/packages/dune)
    - [utop](https://opam.ocaml.org/packages/utop)
- [Rust](https://www.rust-lang.org)
- [SQLite3](https://sqlite.org)
- [Graphviz](http://graphviz.org)

## Instructions

First, you will need to clone this repository to your local filesystem.  You'll only have to do this once this semester (unless you have multiple computers or delete the repository).  To do this, run:

```
git clone https://github.com/anwarmamat/cmsc330spring21
```

The files in the `project0` folder will be used for the [Verifying Setup](#verifying-setup) section at the bottom.

The following sections will help you install the necessary packages and programs on your operating system.  Some steps may take a long time, please be patient.  **Read all instructions very carefully.**

The output of each command is important, please pay careful attention to what each one prints.  If you encounter an error message, do not ignore it.  We will be available in office hours to help you get set up if you run into problems.  As a general rule, no output means the command executed successfully.

**Please skip to the section below that corresponds with your operating system.**

### Linux (NOT WSL)

These instructions assume you have a Debian-based system (e.g. Ubuntu).  If you have a different distribution, you will have to find and download the corresponding packages in your native package manager.  Note that the packages there may have slightly different names.

1. Firstly, install the basic dependencies:
    - Run `sudo apt update` to update your local package listing
    - Run `sudo apt install ruby-dev sqlite3 libsqlite3-dev ocaml ocaml-native-compilers camlp4 make m4 curl graphviz libssl-dev pkg-config`
2. Install some Ruby gems
    - Run `sudo gem install minitest sqlite3 sinatra`
    - If it hangs on `Installing ri documentation for sinatra`, just hit Ctrl+C.  It will have successfully installed anyway
3. Install and initialize the OCaml package manager
    - Run `sh <(curl -sL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)` (when prompted for the installation location, just hit enter to select the default)
        - Run `opam --version`.  You should be on version 2 (followed by some versions, just make sure the major version is 2).  Check out [the manual](https://opam.ocaml.org/doc/Install.html) if this is not the case, you may have to follow special directions for your particular operating system and version.
        - If you encounter any issues, or are running a different flavor of linux, check out [the manual](https://opam.ocaml.org/doc/Install.html)
    - Run `opam init`
    - If it hangs at "Fetching repository information" press Enter. This may take a while, be patient
    - When prompted to modify `~/.profile` (or another file), type "n", but remember the filename
    - Open `~/.profile` (or the file mentioned above) in your text editor
    - Add the line `` eval `opam config env` `` (note these are backticks, not single quotes)
    - Save the file
    - Run `source ~/.profile` (or the file you just edited)
4. Initialize OCaml
    - We will be using OCaml version 4.11.0.  Run `ocaml -version` to check which version is currently installed
    - If you are already on 4.11.0, you can skip to #5
    - Run `opam update`
    - If you are on another version, run `opam switch 4.11.0`.  If you get an error saying that switch is not currently installed, run `opam switch create 4.11.0`.  This may take a while, please be patient
    - Run `eval $(opam env)`
    - Ensure you are now on the correct version by running `ocaml -version`
5. Install OCaml packages
    - Run `opam install ocamlfind ounit utop dune qcheck`
6. Install Rust
    - Go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and run the installation command provided
    - If prompted, just select the defaults
7. Install gradescope-submit
    - Run `cargo install gradescope-submit`.  If this fails, try closing and re-opening your terminal window.

### Windows

*This will only work on Windows 10.  If you are on an older version, you will probably need to set up a Linux VM.*

1. Follow the directions [here](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to install the Windows Subsystem for Linux
2. Install the basic dependencies:
    - Run `sudo apt update && sudo apt upgrade` to update your local package listing
    - Run `sudo apt install ruby-dev sqlite3 libsqlite3-dev ocaml ocaml-native-compilers camlp4 make m4 curl graphviz libssl-dev pkg-config`
3. Install some Ruby gems
    - Run `sudo gem install minitest sqlite3 sinatra`
    - If it hangs on `Installing ri documentation for sinatra`, just hit Ctrl+C.  It will have successfully installed anyway
4. Install and initialize the OCaml package manager
    - Run `sh <(curl -sL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)` (when prompted for the installation location, just hit enter to select the default)
        - Run `opam --version`.  You should be on version 2 (followed by some versions, just make sure the major version is 2).  Check out [the manual](https://opam.ocaml.org/doc/Install.html) if this is not the case, you may have to follow special directions for your particular operating system and version.
        - If you encounter any issues, or are running a different flavor of linux, check out [the manual](https://opam.ocaml.org/doc/Install.html)
    - Run `opam init --disable-sandboxing`
    - If it hangs at "Fetching repository information" press Enter. This may take a while, be patient
    - When prompted to modify `~/.profile` (or another file), type "n", but remember the filename
    - Open `~/.profile` (or the file mentioned above) in your text editor
    - Add the line `` eval `opam config env` `` (note these are backticks, not single quotes)
    - Save the file
    - Run `source ~/.profile` (or the file you just edited)
5. Initialize OCaml
    - We will be using OCaml version 4.11.0.  Run `ocaml -version` to check which version is currently installed
    - If you are already on 4.11.0, you can skip to #6
    - Run `opam update`
    - If you are on another version, run `opam switch 4.11.0`.  If you get an error saying that switch is not currently installed, run `opam switch create 4.11.0`.  This may take a while, please be patient
    - Run `eval $(opam env)`
    - Ensure you are now on the correct version by running `ocaml -version`
6. Install OCaml packages
    - Run `opam install ocamlfind ounit utop dune qcheck`
7. Install Rust
    - Go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and run the installation command provided
    - If prompted, just select the defaults
8. Install gradescope-submit
    - Run `cargo install gradescope-submit`.  If this fails, try closing and re-opening your terminal window.

### macOS

**If you are using an M1 Mac, please follow [https://piazza.com/class/kk79wy51s4z3jg?cid=70](https://piazza.com/class/kk79wy51s4z3jg?cid=70) first.  Then proceed below.**

1. Install the Homebrew package manager
    - Run `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`
2. Check your Ruby version by running `ruby -v`.  If it's older than 2.2.2, you'll need to install a newer version using `brew install ruby`
3. Install the basic dependencies
    - Run `brew install ocaml opam graphviz openssl`
4. Install some Ruby gems
    - Run `sudo gem install minitest sqlite3 sinatra`
    - If it hangs on `Installing ri documentation for sinatra`, just hit Ctrl+C.  It will have successfully installed anyway
5. Initialize the OCaml package manager
    - Run `opam init`
    - When prompted to modify `~/.bash_profile` (or similar file), type "y"
    - Run `source ~/.bash_profile` (or the file mentioned above)
6. Initialize OCaml
    - We will be using OCaml version 4.11.0.  Run `ocaml -version` to check which version is currently installed
    - If you are already on 4.11.0, you can skip to #7
    - Run `opam update`
    - If you are on another version, run `opam switch 4.11.0`.  If you get an error saying that switch is not currently installed, run `opam switch create 4.11.0`.  This may take a while, please be patient
    - Run `eval $(opam env)`
    - Ensure you are now on the correct version by running `ocaml -version`
7. Install OCaml packages
    - Run `opam install ocamlfind ounit utop dune qcheck`
8. Install Rust
    - Go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and run the installation command provided
    - If prompted, just select the defaults
9. Install gradescope-submit
    - Run `cargo install gradescope-submit`.  If this fails, try closing and re-opening your terminal window.

## Verifying Setup

This is the graded part of this project.  To verify that you have the correct versions installed, run `ruby test/public/public.rb` in this directory.  You should not get any errors.  This will create a file called p0.report.  Submit this file by running `gradescope-submit` in the project folder.  You will have to enter your credentials the first time, but for future projects you should not have to.  Alternatively, you can manually submit the file to Gradescope by uploading the p0.report file to the appropriate assignment.
