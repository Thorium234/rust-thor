fn main() {
    // String is a heap-allocated type
    let s1 = String::from("hello");
    
    // This moves ownership from s1 to s2
    let s2 = s1;
    
    // This would cause an error: s1 no longer owns the value
    // println!("{}", s1);
    
    // This works fine
    println!("{}", s2);
    
    // Passing ownership to a function
    takes_ownership(s2);
    
    // Can't use s2 anymore because ownership was transferred
    // println!("{}", s2);
    
    // Primitive types are copied, not moved
    let x = 5;
    let y = x;
    
    // This is fine because integers are copied
    println!("x = {}, y = {}", x, y);
    
    // Borrowing: using references to avoid transferring ownership
    let s3 = String::from("hello");
    let len = calculate_length(&s3);
    
    // s3 is still valid here because we only borrowed it
    println!("The length of '{}' is {}.", s3, len);
    
    // Mutable borrowing
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("Changed string: {}", s4);

//string can be muted
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{s}"); // this will print `hello, world!`


}

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
    // s is dropped (freed) when this function ends
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
    // s goes out of scope, but since it's just a reference,
    // the value it refers to is not dropped
}

fn change(s: &mut String) {
    s.push_str(", world");
}
