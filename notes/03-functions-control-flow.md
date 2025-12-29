# Functions & Control flow in RUST

1) Snake case (lower & underscore seperated) function & variable names. 
2) fn : Keyword to declare function 
3) parameter : identifer : DataType is format to define parameter 
4) Statement : Instruction that performs some action and do not return a value 
5) Expression : Evaluate to a resultant value. 
6) New Scope Block created with curly braces is an expression. Expression do not include semicolon at end. 

## Function with Return Value 

1) Declare return type with fn five() -> i32 { 5 } : '->' used to declare return type 
2) last expression in function block is returned automatically.
3) Early return can be done using return vallues, but most functions return the last expression implicitly  

## Comments 
- // or /**/ : Second option bit less idomatic for actual comment 
- /// & !// : Used for documentation 


## Control Flow 

### if is expression 
- Block of code if last thing is expression those are returned 

```Rust
// Examples 


fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

```

### Loops 
- loop,continue,break normal use for looping over block of code, skipping an execution , breaking out of loop
- break : you can write expression after break & it will return that result of expression so loops are also expression 
  as they can return value. 

``` RUST 
 fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```
- return can be used but it will exit the function not just the loop unlike break.  
- loop lable : we can assign label to loop. They must begin with single quote & in break statement or continue you can define 
  which loop to break or continue to. 

``` RUST 
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

``` 

- 'while' condition { } : for conditional loop running like any other language. 
- for in loop used normal to iterate over collection 

``` RUST 
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

``` 
- (1..4).rev() or Range() can be used on number 
- Todo : write some basic program & rustcane exercise. 
