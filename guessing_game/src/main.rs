use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("----------------------");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", "Guess the number!".cyan().bold());

    loop {
        println!("{}", "\nPlease input your guess.".cyan().bold());

        let mut guess = String::new(); // creates a mutable variable with empty string binded a value. 

        io::stdin() // returns a Type 'Stdin' used to handle input operation in a process
            .read_line(&mut guess) // appends the user input
            .expect("failed to read line");

        // match expression on Result state
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // matches all values of err via _
        };

        println!("\n{} {}", "You guessesd".yellow().bold(), guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red().bold()),
            Ordering::Greater => println!("{}", "Too big!".red().bold()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            }
        }
    }
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
14) Default type infered number is 'u32'
15) We are 'shadowing' guess variable, thus reusing variable name rather than forcing us to create two new variable.
    Used when one type needs to be converted to another type.
16) trim() -> removes whitespace from start & end
17) parse() -> convert string to other data type, type is defined via :u32 thus to variable we are assigning things to.

----------------------------
*/
