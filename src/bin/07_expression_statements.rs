fn main(){
let x:i32={

    let price:i32=5;
    let qty:i32=10;
    price*qty
};
println!("Result is: {}",x);

//unctions returning values
fn add (a:i32,b:i32)->i32{
  a+b   
}

let y= add(4,7);
println!("value of y is:{}",y);
}
