use std::io;
fn main() {
    println!("Greetings app");
    println!("============="); 
    println!("please enter your name:");
    let mut name = String::new();
    let _= io:: stdin()
        .read_line(&mut name);
     println!("Hello,{name} hope you are doing well");   
}
