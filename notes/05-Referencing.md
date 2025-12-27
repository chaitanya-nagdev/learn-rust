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
