# Referencing 

1) Instead of moving owenrship of value we can pass reference of value to function, so no need to return owernship back
2)  __Reference is like a pointer to address, which can be used to access data stored at value__ 
3)  & , * are standard operator used for referencing & dereferencing.  
4) __Think Referencing is giving address or using address of data while Dereferencing as acess data via address.__   
5) The action of passing the referencing instead of ownership is called reference borrowing. Data cannot be modified which is passed 
   as reference. code won't compile  


## Mutable Reference 

1) Like Variable are by default immutable, reference are also immutable by default, so we need to make them mutable in order 
   to modify the data. 
2) If one mutable reference is present no other mutable reference can be present. Similary an immutable reference & mutable reference cannot co-exist 
3) Scope Of Reference is important : It is from declaration to last used, So if a reference is mutable & any other reference used before it goes out of 
  scope compiler will throw error. So multiple references can be created if program is designed such a way at a time only 1 mutable reference is in scope. 
4) Any number of immutable reference can be created as It is read operation they can't change data. 
5) Mutable reference are data writer & thus to prevent data race this feature is introduced. 
6) When Immutable Reference are in scope even owner cannot change the data as it can lead to issue of incorrect data read. 

** Data Race : More than one writer for a value/data & no syncronizing mechnism can lead to data corruption (basically modification in such a way not intended) 
** Dangling Reference : A reference/pointer where no longer data is present, happens is data is dropped & reference is still present in code, Rust prevents 
   at compile time if any such situation arise** 

## Summary 

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.


# Slice Types 

- DST is dynamically sized types. They are actual data whose size is unknown & to be used in program 
- it needs some kind window to be used so a fat pointer (pointer to first element & length) 
- So str & [t] are DST as size is unknow & to be used we need a &str or &[T] 

1) __Now slice in rust is a fat pointer used to read the DST data.__  
2) Slice can be said an immutable refrence to red the contiguous sequecne of element in collection 


## String Slice 

Utill now we know 2 types of string 

1) String literal 
- it is &str so already an slice , actual data is bytes stored in memory & to read it we need fatpointer 
- as it is slice/fat-pointer it is immutable as no ownership of those str 


2) String 
- String has 3 part (pointer to data ,length,capacity), it has ownership of underlying data. 
- when we do &String = &str because underlying data is str only (UTF-8 valid) it just is instead of array[u8] it's stored in vector but actual data is str  


## other slices 
- like string slices, we can have &[T[ so if you define an array of i8 
- So see if you have a type [T] it is dst, we don't know size of array here so we can use Slice to access data as we get length. 
- now confusion was but for array I will know the size ya coorrect but if you pass to function or array comes from 
  input we won't know size so we have to use &[T] so [T,3] or [T,5] array we know the size but for [T] we don't know.
  So we use &[T] to make it generic way to access collection 

Now case of str is unique because it is data type which is UTF-8 valid , it is stored on hard drive as at compile time we won't know size of UTF-8 it can be 1,2,4 

**Note : If you have a collection so you can use Fat-pointer/slice to read underlying data from first elmenet to length (this can be defined using range) only 
  special case for slice types are [T],str which are dst specially introduced to handle string & array as we have literal of this types whose size cannot de determined 
  at compile time and do not reside on stack.**

