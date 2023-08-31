pub fn clear_console() {
	print!("\x1B[2J"); // clear screen
	print!("\x1B[H"); // move cursor to (0, 0)
}
