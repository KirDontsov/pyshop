use crate::stdin;
use crate::utils::clear_console;

pub fn zeros_listener() -> Vec<&'static str> {
	clear_console();
	println!("Сколько символов 0 должно быть в конце строки?");
	let mut zeros_buffer = String::new();
	stdin().read_line(&mut zeros_buffer).expect("Error reading");
	let mut string_on_number_of_zeros = Vec::new();
	let number_of_zeros = zeros_buffer.trim_end().parse::<i32>().unwrap();
	for _ in 0..number_of_zeros {
		string_on_number_of_zeros.push("0")
	}
	string_on_number_of_zeros
}

pub fn lines_listener() -> String {
	clear_console();
	println!("Сколько результатов совпадения вывести?");
	let mut lines_buffer = String::new();
	stdin().read_line(&mut lines_buffer).expect("Error reading");
	lines_buffer
}
