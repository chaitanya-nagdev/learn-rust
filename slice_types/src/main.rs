use std::any::type_name_of_val;

fn main() {
    let s = String::from("hello world"); 


    // PORTION 1 

    let hello = &s[..5];
    let world = &s[6..]; // slice of str (str is like primitive i8, it is valid UTF-8 byte so u8, it just we dont know the length
                         // at compile time but we cannot change it, it cannot grow once allowed. 

    println!("s is: {}", type_name_of_val(&s)); // expects immutable reference to variable -> thus
                                                // printing String 
    println!("hello is: {}", type_name_of_val(&hello)); // receives &&str -> print &str
    println!("world is: {}", type_name_of_val(world)); // receives &str -> print str 
                                                       
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..];


    println!("a is: {}", type_name_of_val(&a)); // receives &[i32,5] reference to array of fixed
                                                // size print that 
    println!("slice is: {}", type_name_of_val(slice)); // receives &[i32] so a slice type thus
                                                       // print i32
    // PORTION 2 
    // test_str(s); : mismatches types  we are passing String  but expecting is &str 
    test_str(&s); // valid &s is &str as &String -> &str 
}


fn test_str(s:&str) {
    println!("Yo {}",s);
}
