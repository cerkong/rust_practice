use std::io;

fn main() {

    loop {

        println!("\nPress q to exit.");
        println!("Please enter temperature in Fahrenheit or Celsius (33.2C or 67F):");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature. Please try again.");

        let temp = temp.trim();

        if temp == "Q" || temp == "q" {
            println!("\nProgram successfully exited.");
            break;
        }

        let (number, unit) = temp.split_at(temp.len() - 1);

        let number: f64 = match number.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        match unit.to_uppercase().as_str() {

            "F" => println!("{:.2}F = {:.2}C", number, (number - 32.0) * 5.0 / 9.0),
            "C" => println!("{:.2}C = {:.2}F", number, (number * 9.0 / 5.0) + 32.0),
            _ => println!("Please end with F or C, try again."),
        }

    }
}
