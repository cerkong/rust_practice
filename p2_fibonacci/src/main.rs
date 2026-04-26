use std::io;

fn main() {

    loop {

        println!("\nPress q to exit.");
        println!("Please enter the Fibonacci number:");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number. Please try again.");

        let number = number.trim();
        if number == "Q" || number == "q" {
            println!("\nProgram successfully exited.");
            break;
        }

        let number: u128 = match number.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again with a non-negative integer.");
                continue;
            }
        };

        println!("Fibonacci number: {}", fibonacci(number));
    }
}

fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}