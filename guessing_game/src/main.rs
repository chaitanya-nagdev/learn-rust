use colored::*;
use std::io;

fn main() {
    println!();
    println!();
    println!("----------------------");
    println!("{}", "Guess the number!".cyan().bold());
    println!("{}", "Please input your guess.".cyan().bold());

    let mut guess = String::new(); // creates a mutable variable with empty string binded a value. 

    io::stdin() // returns a Type 'Stdin' used to handle input operation in a process
        .read_line(&mut guess) // appends the user input
        .expect("failed to read line");

    println!(
        "\n{} {}",
        "You guessesd".yellow().bold(),
        guess.yellow().bold()
    );
}

/*

// Foot Notes

1) std : Standard Library
2) io : input/output library
3) prelude : set of items every rust program brings into scope
4) A type which is not a 'prelude' and a program needs it, we have 'use' keyword to bring
  those types into scope

5) println! : It a macro to print things
6) let identifer = value : create a 'immutable' variable & binds it to 'value'
7) let mut identifier = value : create a 'mutable' variable & binds it to 'value' thus this variable can change.
9) String::new() : Returns a new empty string, String is data type provided by standard library which is
    growable. note it stores 'UTF-8' characters

10) :: syntax in String::new indicate 'new is an "associated function"'.
    Associated Function : A function which is attached to type.

11) Result : It is enumeration (enum) : A type which can have multiple state. A possible state is
    refered to as variant
    Result : As this is type, it has associated function with it. expect function if Result is 'Err'
            value then it will crash the program. If 'Ok' it will return the value 'Ok' is holding
            thus value can be used later.
12) {} : empty brackets display the value in order they are defined in println()! macro.


13) Importing crate : Add in 'dependency', bring in scope using 'use'


----------------------------
*/
