## **Variable**
A named container that stores a value in memory.  
Example: `let x = 5;`

---

## **Function**
A reusable block of code that performs a specific action.  
Takes input (parameters) and often returns output.

---

## **Expression**
Code that evaluates to a value.  
Examples: `3 + 4`, `"hello".len()`, `x * 10`.

---

## **Statement**
A complete instruction that performs an action.  
Example: `let name = "Polyvios";`

---

## **Conditional**
A structure that chooses different paths based on a condition.  
Common forms: `if`, `else`, `else if`, `match`.

---

## **Macro**
A macro is a construct that expands into code **before** compilation.  
Unlike functions, which run at runtime, macros generate or transform code at compile time.

Macros are used to:
- reduce repetitive code  
- create code from patterns  
- accept variable numbers of arguments  
- perform metaprogramming  

Rust macros always end with an exclamation point:

```rust
println!("Hello");
format!("Result: {}", x);
vec![1, 2, 3];

---

## **Stack Memory**
Fast memory used for small, short-lived data and function calls.

---

## **Heap Memory**
Slower memory used for dynamic, growable data.  
Values placed here are accessed via references or pointers.

---

## **Pointer / Reference**
A value that “points to” a memory location rather than storing data directly.  
Rust uses `&value` and `&mut value`.

---

## **Ownership** *(Rust-specific but foundational)*
Rust’s system ensuring memory safety without garbage collection.  
Rules:
1. Each value has a single owner  
2. Ownership moves  
3. Value is dropped when owner goes out of scope  

---

## **Borrowing** *(Rust)*
Accessing data without taking ownership.  
Immutable (`&x`) or mutable (`&mut x`) borrows.


