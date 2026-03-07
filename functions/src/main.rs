fn main() {
    println!("Hello, world!");

    hello_world();
    tell_height(182);
    human_details("thorium",55,182.0);
}

fn human_details(name:&str,age:u32,height:f32){

    println!("my name is {}, im {}years old and my height is {} cm.",name,age,height);
}

fn hello_world(){
 println!("hello, Rust!");
}

//hoisting-where you define and call function
//
//inserting value 
fn tell_height(height:u32){
 println!("my height is {}",height);
}
