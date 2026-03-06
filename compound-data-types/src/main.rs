fn main() {
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

}
