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
fn main(){
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


 //lesson 2
 //compound data types 
 //arrays,tuples,slices, and strings(slice string)
 //arrarys
 let a = [1, 2, 3, 4, 5];
 let first = a[0];
 let _second = a[1];
 println!("array 1: {}",first);
 //example2 
 let number : [i32;5]=[1,2,3,4,5];
 println!("number Array: {:?}",number);
 
 //types of formeters
 //a) debuggable eg {:?}
 //b) display eg {}
 let fruits:[&str;3]=["bananas","apple","potatos"];
     println!("fruits array: {:?}",fruits);
     println!("item 1:{}",fruits[0]);//banana

 //tuples
 //let tup: (i32, f64, u8) = (500, 6.4, 1);
//
 let tup = (500, 6.4, 1);
 let (_x, _y, _z) = tup;
 println!("The value of y is: {y}");
//exampe2
let human = ("alice",30,true);
println!("human tuple: {:?}",human);
// N/B to convert a string like "alice" to string slice we use .to_string() keyword eg
// "bananas".to_string()


//slices
//good for memory allocation and efficiency
let number_slice: &[i32]=&[1,2,3,4,5];
println!("number slice:{:?}",number_slice);

//strings vs string slice(&str)
//string are mutable;they are owned;are grawable are stored in a heap->wich can be shrinked or
//expanded dynamically; strings are slow
//
//example 
//any data inm rust is immutable you can make it mutable by adding `mut` keyword 
let mut stone_cold: String = String::from ("hello, ");
println!("stone says:{}",stone_cold);
stone_cold.push_str("yeah!");
println!("stone says:{}",stone_cold);
//you have noted this is a heap 

//&str (string slice)
//data is stored in a stack is quick to access
//data is immutable
//no ownership
//slice refers to the values
//example
let string: String=String::from("hello,");
let slice:&str=&string;//&str[0..5]
 println!("slice values: {}",slice);


//rust compiler cleans memory automatically hence yo cant access a function outside main function
//functions
//entry points
//a function, variables should be writen in snakecase
//naming convenions in rust is:
//1. snake_case: hello_world
//2. kebab case : hello-world

 //new function
 hello_world();
 //hoisting-> where do we define a function and call it
tell_height(122);
human_details ("thrium",55,182.0);
}
fn hello_world(){
 println!("hello, rust!");

}
//inserting values
fn tell_height(height:u32){
 println!("my height is:{}",height);

}

//you can insert more than one value 
fn human_details(name:&str,age:u32,height:f32){
 println!("my name is{},i am {} years old, and my height is {} cm.",name,age,height);
}



