use std::env;
use std::fs;

const DEFAULT_TEST_PATH: &str = "test-95.txt";

fn main() {
	let args: Vec<String> = env::args().collect();

	let spaces = match &args[..] {
		[_, cmd] => parse_input(cmd, None),
		[_, cmd, path] => parse_input(cmd, Some(path)),
		_ => 0,
	};

	println!("number of spaces: {}", spaces);
}

fn parse_input(cmd: &String, path: Option<&str>) -> i32 {
	let mut spaces = 0;
	let contents = fs::read_to_string(path.unwrap_or_else(|| DEFAULT_TEST_PATH)).unwrap();
	match cmd.as_str() {
		"iter" => {
			spaces = test_iter(&contents);
		}
		"loop" => {
			spaces = test_loop(&contents);
		}
		"for" => {
			spaces = test_for(&contents);
		}
		_ => (),
	}
	spaces
}

///  Benchmark 1: ./target/release/rs-tests iter
///  Time (mean ± σ):     410.6 ms ±   2.8 ms    [User: 358.8 ms, System: 50.1 ms]
///  Range (min … max):   406.6 ms … 417.5 ms    100 runs
fn test_iter(contents: &String) -> i32 {
	let mut spaces = 0;
	for (_, c) in contents.chars().into_iter().enumerate() {
		match c {
			' ' => spaces += 1,
			_ => (),
		}
	}
	spaces
}

///  Benchmark 1: ./target/release/rs-tests loop
///  Time (mean ± σ):     451.4 ms ±   2.5 ms    [User: 399.3 ms, System: 50.7 ms]
///  Range (min … max):   445.2 ms … 457.5 ms    100 runs
fn test_loop(contents: &String) -> i32 {
	let mut spaces = 0;
	let mut it = contents.chars().into_iter().enumerate();
	loop {
		match it.next() {
			Some((_, x)) => match x {
				' ' => spaces += 1,
				_ => (),
			},
			None => break,
		}
	}
	spaces
}

///  Benchmark 1: ./target/release/rs-tests for
///  Time (mean ± σ):     262.4 ms ±   3.7 ms    [User: 210.0 ms, System: 50.9 ms]
///  Range (min … max):   258.0 ms … 284.4 ms    100 runs
fn test_for(contents: &String) -> i32 {
	let mut spaces = 0;
	for i in 0..contents.len() {
		match &contents[i..i + 1] {
			" " => spaces += 1,
			_ => (),
		}
	}
	spaces
}
