//Basic Syntax and Concepts
//Now that we've set up Rust and created our first program, let's explore some basic Rust syntax and concepts.

//Variables and Mutability 

//let x = 5;
//print!("the value of x is: {}",x);
//the line below will couse a compilation error
//x= 6;
//to make a variable mutable,use the `mut` keyword
//let mut y = 5;
//println!("the value of y is:{}",y);
//y=6;
//println!("the value of y is now:{}",y);
//lesson_1
    //primitive data types
    //int,float,bool,char
    //
//integer
    //Rust has signed(+ and -) and unsigned integer(only+) types of different sizes
    //i8,i16,i32,i64,i128:signed integers.
    //u8,u16,u32,u64,u128: ubsigned integers.

fn main() {

    println!("Hello, world!");
    let x:i32 = -433;//the value can be positive or negative
    let y:u64 = 100;//only positive

    println!("signed int: {}",x);
    println!("unsigned int {}",y);

    //difference between i32 (32 bits) and i64(64bits)
    //range:
    //i32-2147483647
    //i64-9223372036854775807
    //this are the largest possible value

//floats [floating point types]
//f32,f64
 let pi:f64 = 3.14;
 println!("value of pi: {}",pi);

 //Boolean values
 //true,false
 let is_snowing: bool = true;
 println!("its snowing: {}",is_snowing);

 //Character type- char
 //x,z
 let letter: char = 'A';
 println!("first letter of alphabet: {}",letter);

}

