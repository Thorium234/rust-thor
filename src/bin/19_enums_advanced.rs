 #[allow(dead_code)]
#[derive(Debug)]
enum Colors {
    RED,
    GREEN,
    BLUE,
}

fn main() {
    let my_color = Colors::BLUE;
    println!("Color: {:?}", my_color);
}
