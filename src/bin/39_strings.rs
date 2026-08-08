fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s2 = data.to_string();

    let s3 = String::from("initial contents");

    let mut s4 = String::from("foo");
    s4.push_str("bar");

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // s5 has been moved here and can no longer be used

    println!("{s7}");

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");

    let s_format = format!("{s8}-{s9}-{s10}");
    println!("{s_format}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
