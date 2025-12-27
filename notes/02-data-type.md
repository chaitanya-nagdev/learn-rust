# Data Types in Rust 

- Data Types SubSet Types 
    - Scalar 
    - Compound 

## Scalar types 
- Scalar types represent an single value 
- Primary 4 Types

1) Integer 

| Bit Length | Signed Type | Unsigned Type |
|-----------:|------------|---------------|
| 8-bit      | i8         | u8            |
| 16-bit     | i16        | u16           |
| 32-bit     | i32        | u32           |
| 64-bit     | i64        | u64           |
| 128-bit    | i128       | u128          |
| Arch-dependent | isize  | usize         |

Signed Range : −(2^n − 1) to 2^(n − 1) − 1
UnSigned Range : 0 to 2^n − 1

- Literals we can append u8,u32.. 
- we can use _ as seperator while writing integer for clarity  

2) Floating point 

- f32,f64 : 32,64 bits floating point numbers, 64 bit is more precise
- Use IEEE-754 (Read this once)

3) Booleans 
- 1 byte 
- true or false  
- Syntax : let f: bool  = false; 

4) Characters 
- Alphabetic 
- 4 bytes in size 
- Unicode scalar : Thus more than english alaphabets 
- Range : (U+0000 to U+D7FF) and ( U+E000 to U+10FFFF) inclusive.  
- Syntax : let z:char = 'ℤ'; 


## Compound Types 
- This types group mutliple values 


1) Tuples Type 
- The have fixed length : Once declared, they cannot grow or shrink in size. 
- Used for grouping together a number of values with variety of types into one compound type. 

```Rust 
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

- Destructing is possible via assignment to mulitple types. 
- We can access using '.' & index. 
- **Tuple without any value has special name unit. Written as () both value & type. Expression returns this unit if they don't return a value  ** 



```

2) Array Element Access 
- Fixed Length 
- Same types of values can be grouped.
- Syntax :
    - let a = [1, 2, 3, 4, 5];
    - let a: [i32; 5] = [1, 2, 3, 4, 5];
    - let a = [3; 5]; // 5 values of integer 3 is stored in array 
- Stored on stack. Cannot grow or shrink & size known at compile time.  
- Panics : In case index used for access data is invalid (greather or equal to length of array) 
  
