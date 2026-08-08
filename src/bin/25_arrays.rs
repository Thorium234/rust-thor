fn main() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];

    println!("The first element is: {first}");
    println!("The second month is: {}", months[1]);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3, 3, 3, 3, 3]

    println!("c[0] is: {}", c[0]);
}
