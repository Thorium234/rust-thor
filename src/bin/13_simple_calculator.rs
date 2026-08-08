use std::io;

fn main() {
    loop {
        println!("Enter expression (e.g., 2+3 or 2 + 3) or q to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed");

        let input = input.trim();

        if input == "q" {
            break;
        }

        match evaluate(input) {
            Some(result) => println!("Result: {}", result),
            None => println!("Invalid expression"),
        }
    }
}

fn evaluate(input: &str) -> Option<f64> {
    let operators = ['+', '-', '*', '/'];

    for op in operators {
        if let Some(idx) = input.find(op) {
            let left = input[..idx].trim();
            let right = input[idx + 1..].trim();

            let a: f64 = left.parse().ok()?;
            let b: f64 = right.parse().ok()?;

            return match op {
                '+' => Some(a + b),
                '-' => Some(a - b),
                '*' => Some(a * b),
                '/' => {
                    if b != 0.0 {
                        Some(a / b)
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }
    }

    None
}
