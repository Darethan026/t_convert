# t_convert

This is a rust library to convert temperatures between Celsius, Fahrenheit, and Kelvin, without going below absolute zero.

To use it, copy and paste 'cargo add t_convert' on the command-line in the project directory and it will be added to your project.

# Example

```rust
use t_convert::{Temperature, Unit};
use std::io;

fn main() {
	println!("Type in to convert from celsius to kelvin\n");
	
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("Failed to read line");

	match input.trim().parse::<f64>() {
		Ok(num) => {
			match Temperature::new(num, Unit::Celsius) {
				Some(temp) => {
					if let Some(kelvin) = temp.to_kelvin() {
						// Use get_value() to show access the result safely.
						println!("\n{} degrees Celsius is {} kelvin.", temp.get_value(), kelvin.get_value());
					}
				}

				None => {
					println!("\nError: {} is below absolute zero!", num);
				}
			}
		}

		Err(e) => {
			println!("Error: ({})", e);
		}
	};
}
```
If you still aren't sure on how to use the library, refer to 'https://docs.rs/t_convert' to see the code.