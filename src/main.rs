use crate::thread_spawner::thread_spawner;
use std::io::stdin;
use std::time::Instant;

mod input;
mod thread_spawner;
mod utils;

type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let _start_time = Instant::now();

	let res = thread_spawner();

	res.lock()
		.unwrap()
		.iter()
		.for_each(|x| println!("{}, {:X?}", x.0, x.1));
	let start_elapsed = _start_time.elapsed().as_millis();
	println!("Search time: {} ms", start_elapsed);
	Ok(())
}
