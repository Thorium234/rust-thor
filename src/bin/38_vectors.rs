fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third_option: Option<&i32> = v2.get(2);
    match third_option {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }

    for i in &v3 {
        println!("{i}");
    }
}
