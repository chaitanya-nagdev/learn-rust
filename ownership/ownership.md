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
- 
