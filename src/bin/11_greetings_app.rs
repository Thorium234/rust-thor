use std::io;
fn main() {
    println!("Greetings app");
    println!("============="); 
    println!("please enter your first  name:");
    //println!("please enter your second name:");
    let mut fname = String::new();
   // let mut lname = String:: new();
    let _= io:: stdin()
        .read_line(&mut fname);
    println!("please enter your second name: ");
    let mut lname = String:: new();
    let _= io::stdin()
        .read_line(&mut lname);
     println!("Hello,{fname}{lname} hope you are doing well");   
}
