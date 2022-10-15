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
		"iter" => spaces = test_iter(&contents),
		"for" => spaces = test_for(&contents),
		"bytes" => spaces = test_bytes(&contents),
		_ => (),
	}
	spaces
}

///  Benchmark 1: ./target/release/rs-tests iter
///  Time (mean ± σ):     259.2 ms ±   2.4 ms    [User: 225.7 ms, System: 32.6 ms]
///  Range (min … max):   254.8 ms … 265.3 ms    100 runs
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

///  Benchmark 1: ./target/release/rs-tests for
///  Time (mean ± σ):     165.4 ms ±   3.6 ms    [User: 131.4 ms, System: 32.8 ms]
///  Range (min … max):   161.8 ms … 181.2 ms    100 runs
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

///  Benchmark 1: ./target/release/rs-tests for
///  Time (mean ± σ):      59.0 ms ±   2.9 ms    [User: 25.2 ms, System: 33.2 ms]
///  Range (min … max):    55.7 ms …  65.2 ms    100 runs
fn test_bytes(contents: &str) -> i32 {
	let mut spaces = 0;
	for (_, b) in contents.bytes().enumerate() {
		match b {
			b' ' => spaces += 1,
			_ => (),
		}
	}
	spaces
}
