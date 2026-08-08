// *********=> INTEGER, STRING, TUPLE, VECTOR, FUNCTION, OWNERSHIP, BORROWINGA

fn main() {
    // *********=> INTEGER (Copy type, lives on stack)
    let mut num: u8 = 5;
    // let num = 5; // Rust can infer the type for simple numeric values
    println!("value stored in num: {}", num);
    num = 2; // to make a variable mutable, add `mut` in front of the name
    println!("value stored in num: {}", num);

    // *********=> STRING (heap-allocated)
    // let string_literals: &str = "HELLO WORLD"; // &str = string slice (string literal)
    let string = String::from("HELLO WORLD"); // heap-allocated growable string
    println!("string: {}", string);

    // *********=> TUPLE
    let emp_info: (&str, u8) = ("Ramesh", 50);
    let (emp_name, emp_age) = emp_info;
    println!("Employee name: {}, age: {}", emp_name, emp_age);
    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;

    // *********=> FUNCTION CALLING (Copy types)
    let num1: u8 = 10;
    let num2: u8 = 20;

    let result: u8 = print_value(num1, num2);
    println!("result: {}", result);
    // num1 and num2 are still valid here because u8 implements Copy.

    // *********=> OWNERSHIP BASICS
    ownership_func();
    ownership_func2();

    // *********=> ARRAYS AND VECTORS
    array_fn();
    vector_fn();

    // *********=> SHADOWING
    shadowing();
}

// add parameter type int,string
fn print_value(num1: u8, num2: u8) -> u8 {
    num1 + num2
}

// *********=> OWNERSHIP: SCOPE

// global scope
const GLOBAL_CONST: u8 = 5; // const requires type annotation and should be UPPERCASE

fn ownership_func() {
    println!("GLOBAL_CONST: {}", GLOBAL_CONST);

    // function scope
    let outside_variable = 5;

    // block scope
    {
        let inside_variable = 5;
        println!("INSIDE VARIABLE: {}", inside_variable);
    }

    // inside_variable is not accessible here
    println!("OUTSIDE VARIABLE: {}", outside_variable);
}

fn ownership_func2() {
    // This works because integers are Copy types (stack-allocated, small, trivial)
    let a = 5;
    let b = a; // a is copied
    println!("a: {}, b: {}", a, b);

    // For heap types like String, ownership is moved, not copied by default
    let str1: String = String::from("HELLO"); // str1 owns "HELLO"
    let str2 = str1; // ownership moved from str1 to str2

    // println!("{}", str1); // ❌ error: str1 is no longer valid here
    println!("str2: {}", str2);

    let x: String = String::from("HELLO BYE");
    // transfer ownership of x into function new_ownership_func
    new_ownership_func(x);
    // println!("{}", x); // ❌ error: x was moved
}

fn new_ownership_func(item: String) {
    println!("item (taken by ownership): {}", item);

    let mut s: String = String::from("beyeyyeee"); // s owns this String

    // Borrowing mutable reference to s
    let len: usize = new_ownership_func2(&mut s); // passing &mut s (borrow, not move)
    println!("VALUE of s: '{}', len before push: {}", s, len);

    // *********=> REFERENCE RULES

    let mut s1: String = String::from("Hello");

    {
        // First mutable borrow in this inner scope
        let w1 = &mut s1;
        w1.push_str(" World");
        println!("w1 = {}", w1);
    } // w1 goes out of scope here, so we can borrow mutably again

    {
        // Second mutable borrow in a separate scope
        let w2 = &mut s1;
        w2.push_str(" Again");
        println!("w2 = {}", w2);
    } // w2 goes out of scope

    println!("Final s1: {}", s1);

    // You cannot have two active &mut references at the same time.
    // This would be an error:
    //
    // let r1 = &mut s1;
    // let r2 = &mut s1; // ❌ cannot borrow `s1` as mutable more than once at a time
}

fn new_ownership_func2(item: &mut String) -> usize {
    let length = item.len();
    item.push_str("dsvndfvdv"); // modify through mutable reference
    new_main();
    length
}

fn new_main() {
    // *********=> REFERENCE AND DEREFERENCE
    let mut x = 5;
    x = x + 1;

    let y = &mut x; // y is a mutable reference to x
    *y = *y + 1;    // use * to dereference and modify the underlying value

    println!("y (mutable ref to x): {}", y);
}

fn array_fn() {
    // *********=> ARRAY
    // fixed-size, same-type, lives on the stack
    let mut arr: [&str; 3] = ["HELLO", "BYE", "CODER"];

    // Here arr is copied into arr1 (arrays of Copy types are Copy up to a certain size)
    write_mut_fn(arr);
    // Here we pass a mutable reference to the original arr
    write_ref_fn(&mut arr);

    println!("arr in array_fn (after write_ref_fn): {:?}", arr);
}

fn write_mut_fn(mut arr1: [&str; 3]) {
    // arr1 is a copy of arr and is local to this function
    arr1[0] = "FELLOW";
    println!("arr1 in write_mut_fn: {:?}", arr1);
}

fn write_ref_fn(arr2: &mut [&str; 3]) {
    // We are modifying the original arr via mutable reference
    arr2[0] = "FELLOW2";
    println!("arr2 in write_ref_fn: {:?}", arr2);
}

fn vector_fn() {
    // *********=> VECTOR (heap-allocated growable array)
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Pass a mutable reference to the vector (no clone, no move of ownership)
    write_vector_mut_fn(&mut v);
    println!("v after write_vector_mut_fn: {:?}", v);
}

fn write_vector_mut_fn(v: &mut Vec<i32>) {
    v.push(99);
    println!("v inside write_vector_mut_fn: {:?}", v);
}

fn shadowing() {
    // *********=> SHADOWING
    let x = 5;
    println!("value of x: {}", x);
    let x = 10; // shadows previous x
    println!("value of x after shadowing: {}", x);
}
