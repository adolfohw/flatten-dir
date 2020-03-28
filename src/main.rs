//! This simple CLI flattens a directory recursively.
//!
//! # Usage:
//!
//! ```sh
//! flatten_dir <path>
//! ```

use flatten_dir::flatten;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
	let dest = PathBuf::from(match env::args().nth(1) {
		Some(path) => path,
		None => {
			println!("You must provide a path to be flattened");
			return;
		}
	});
	let canonical = dest.canonicalize();
	let full_path = if dest.is_file() || canonical.is_err() {
		println!("Invalid directory path");
		return;
	} else {
		canonical.unwrap().to_string_lossy().to_string()
	};
	let human_full_path = if full_path.starts_with(r"\\?\") {
		&full_path[r"\\?\".len()..]
	} else {
		&full_path[..]
	};
	println!("You are about to flatten `{}`", human_full_path);
	let mut ans = String::with_capacity(1);
	loop {
		print!("Are you sure you want to proceed? [y/n] ");
		io::stdout().lock().flush().unwrap();
		if let Ok(_) = io::stdin().read_line(&mut ans) {
			match ans.chars().nth(0) {
				Some('y') | Some('Y') => break,
				Some('n') | Some('N') => return,
				_ => ans.clear(),
			}
		}
	}
	if let Err(err) = flatten(dest) {
		println!("Failed to flatten `{}`", human_full_path);
		println!("{}", err);
	}
}
