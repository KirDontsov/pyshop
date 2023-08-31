use crate::input::{lines_listener, zeros_listener};
use sha256::digest;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn thread_spawner() -> Arc<Mutex<Vec<(String, String)>>> {
	let string_on_number_of_zeros = zeros_listener();
	let lines_buffer = lines_listener();

	// потокобезопасный указатель
	let res = Arc::new(Mutex::new(Vec::new()));

	// потокобезопасный указатель
	let pair = Arc::new((
		Mutex::new(string_on_number_of_zeros.join("")),
		Mutex::new(lines_buffer.trim_end().parse::<i32>().unwrap()),
	));

	// сюда будем складывать потоки
	let mut handles = vec![];
	// берем максимальное число доступных ядер
	let cpu_count = thread::available_parallelism()
		.expect("Error cpu count")
		.get();

	for i in 1..=cpu_count {
		let zeros_and_lines_clone = Arc::clone(&pair);
		let res_clone = Arc::clone(&res);

		let handle = thread::spawn(move || {
			let zeros = zeros_and_lines_clone.0.lock().unwrap();
			let lines = zeros_and_lines_clone.1.lock().unwrap();
			let mut res = res_clone.lock().unwrap();

			// для ускорения обработки при маленьких запросах уменьшаем выборку
			let range = if zeros.len() < 6 || *lines < 12 {
				1_000_000
			} else if zeros.len() < 7 || *lines < 6 {
				10_000_000
			// при необходимости увеличить верхний порог
			} else {
				100_000_000
			};

			// разбиваем диапазон по 1_000_000 на ядро
			let min = if i == 1 { 1 } else { (i - 1) * range + 1 };
			let max = if i == 1 { i + range - 1 } else { i * range };
			println!("Thread {:?} started with {:?}-{:?}", i, min, max);

			for j in min..=max {
				// выходим из цикла если нашлось нужное кол-во записей
				if res.len() == *lines as usize {
					break;
				}
				// хэшируем число
				let val = digest(j.to_string());
				// если число заканчивается на заданное кол-во 0, добавляем его в res
				if res.len() != *lines as usize && val.ends_with(&*zeros) {
					res.push((j.to_string(), val));
				}
			}
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}
	res
}
