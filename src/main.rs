fn main(){


 //new function
 hello_world();
 //hoisting-> where do we define a function and call it
tell_height(122);
human_details ("thrium",55,182.0);


//========================expressions and statements================================
//expressions are anything that returns a value
//statements are anything that does not return a value
//======examples of expressions========
//5
//true or false
//add(3,4)
//if conditions
//=====================================================
// we should use const and static when declaring a global variable instead of using `let`
// we use const and UPPER_SNAKE_CASE
// eg const _X ={
// code
// }
// ==================example1 
let x:i32={
let price:i32=5;
let qty:i32=10;
price*qty //you may use return price*qty;

};
println!("result is:{}",x);

//functions returning values
fn add(a:i32,b:i32)->i32{

a+b
}
let y = add(4,6);
println!("values of y is: {}",y);

//===============statements================
//they dont return a value
//almost all statements in rust ends with ;
//variables diclaration are ststements eg let x = 5;
//examples of statements
//function definition fn foo() {}
//control flow statements id contitions
//calling BMI
let weight:f64=89.87;
let height:f64=1.353;
let bmi:f64 = calculate_bmi(weight,height);
println!("your bmi is: {:.2}",bmi);


}
//=============================outside main function================

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

//BMi= height(kg)/height(m)^2
fn calculate_bmi(weight_kg:f64,height_m:f64)->f64{
weight_kg/height_m*height_m
}

