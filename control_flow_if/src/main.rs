fn main() {
    // If-else
    let number = 7;
    
    if number < 5 {
        println!("Number is less than 5");
    } else if number > 10 {
        println!("Number is greater than 10");
    } else {
        println!("Number is between 5 and 10");
    }
    
    // Loops
    let mut counter = 0;
    
    loop {
        counter += 1;
        
        if counter == 5 {
            break;
        }
        
        println!("Loop iteration: {}", counter);
    }
    
    // While loop
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");
    
    // For loop
    let numbers = [10, 20, 30, 40, 50];
    
    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}
