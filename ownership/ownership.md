# Ownership Important Points 

1) Each Langauge has to manage memory of program. Some use garbage collector freeing memory which is no longer being used,
   while other languages use explicit allocation & deallocate of memory for the program. 

   Rust uses third approach : "Memory is managed through system of ownership with a set of rules that compiler 
                               checks,if any rule is violated, program won't compile" 
---  
## Stack And Heap 

- Stack & Heap both are part of memory available to code to use at runtime. But structuring of data is 
  different in both of them. 
- Stack : LIFO (Last In First Out), Stores the value in order it gets, but removing in opposite order. Pushing,Popping are 
          term used for adding, removing onto the stack. ___All data being stored on the stack must have known fixed size. 
           Any Data with unknown size at compile time or size that might change is stored in heap___. 

- So Processor  already knows how to access data & what is the size thus it fast. 
- Heap : It is less organized, When data is stored in heap, processor find big enough space mark the memory as in use & return         location of memory & location is stored in pointer. This whole process is called 'allocation'. 
         This whole process is called 'allocation'. This whole process is called 'allocation'. This whole process is called 
        'allocation'. Size of pointer is known thus it is stored on heap & then using that pointer we can access the memory.

- allocating & access the memory on stack is faster & on heap is slower in comparision, because allocation on heap 
  needs t ofind couple of things while stack stores at top, similary memory jump for processor is less in stack & that 
  makes it faster, the less memory jump, faster processor. 

- So when function call happens, local variable & arguments are stored (pointer) on stack & once call is finished 
  values get popped off the stack. 

- **So "ownership solves" : Keep track What part of code uses what data on heap,
  minimizing the amount of duplicate data on heap and cleaning up unused data on the heap, 
  so we don't run out of space are problem that ownership address.** 

---

## Ownership Rules 

1) Each value in Rust has owner. 
2) There can be only one owner at a time.  
3) When owner get's out of scope, the value will be dropped.


### Variable Scope 

- Scope is the range within a program for which item is valid  
- When scope comes into scope, it is valid & it remains valid until it goes out of scope. 

### 'String' Type 

- This is type which is stored at heap like many different data type, we can use String::from to create a new string which 
  size is unknow at compile time & is stored at heap. 
- Using String::from -> String can be mutated & literal cannot becuase 
- __Rust Calls 'drop' function when a variable goes out of scope__ : This frees up the memory request/allocated at runtime of program. 
  now this is similar to RAII pattern of C++, generally when we manually deallocate memory we have to make sure we allocate & deallocate 
  memory at same time. 

- For int variable we know size at compile time & thus it's stored at stack, for String that's not the case, this is how string is stored 
  3 parts 
  1) Capacity : Total Memory allocater has given 
  2) Length : Current size of content in bytes 
  3) Pointer : Address of memory where actual content are stored on heap 

  Heap : Stores actual content. 
  Thus 2 part (Data,Content) 

- When S1=S2 , we copy data thus pointer,capacity,length not actual content, now main issue is 
  When S1,S2 goes out of scope it free up memory twice which needs to memory corruption & leads to security vulnerabilities 
  So what Rust does is __Makes S1 as invalid & thus does not free memory twice__ , if you try to use it then it will throw compile 
  time error. So basically 'it moves the data from S1 to S2' in other programming we have shallow & deep copy but as we are making 
  S1 invalid, we say moved & there it implies __rust never automatically creates "deep" copy__ & this whole things solves double freeing 
  memory issue. 

- ___Another Important Point : So suppose a value is being bind to variable & then you bind another value with it, now first value is not 
  being pointed by anyone & is out of scope thus it is dropped.___ 

- Programmer can create deep copy via clone() method on values indicating data is being copied & points to something different is going on. 
- Stack Only Data : Copy : They are copied like integer because size is known at compile time & no difference between shallow & deep copy. 
  Rust has __Copy__ trait which can be implement by types , then data is not moved but trivially copied. 
- A type cannot implement Copy trait if it implements Drop trait & any part of type implement Drop trait, programmer can see type which has 
  implemented Copy trait but general rule is 'Scaler' types do not allocate Copy 
- Rules with Funciton, just like Variable assignment function call moves/copies the ownership/data to function & if function returns it return 
  the ownership basically if it goes out of scope without returning ownership value goes out of scope but doing this return everything is 
  tedious & verbose & thus rust has concept of __"Referencing"__ 


  Summary 

  1) Values Stored at heap are moved from one variable to another & not copied like shallow or deep. 
  2) Values which are no longer referencing are gone out of scope so does matter if variable is still in scope which was pointing 
     to value if value is not longer binded, it goes out of scope & drop function is called 
  3) Variable which moved data then old variable are no longer valid & throws compile time error if used. 
  4) Function call behaves same as Variable assignement in terms of moving & coping, stack data is copied via Copy trait & some 
     rules of Copy trait can also be kept in mind. 
  5) Referencing feature : This allows to pass value wihtout giving ownership for data allocated at heap 
    



