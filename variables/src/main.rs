fn main(){

    let name = "thorium234";
println!("My first name is: {}", name);

let name = "thorium234";
let age = 30;
println!("{} is {} years old.", age, name);  // Outputs 30 is thorium234 years old
                                             //

let x = 5;
x = 10; // Error
println!("{}", x);

let mut x = 5;
println!("Before: {}", x);
x = 10;
println!("After: {}", x);
}
