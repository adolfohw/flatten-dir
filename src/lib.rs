use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

/// Flattens a directory recursively, moving all nested files into `path`.
///
/// # Errors
///
/// This will fail if `path` is not a directory or if some intermediate
/// I/O operation fails.
///
/// # Example
///
/// ```norun
/// flatten("./music_lib_from_2001").unwrap();
/// ```
pub fn flatten<P>(path: P) -> io::Result<()>
where
	P: AsRef<Path>,
{
	flatten_or_move(&path, &path)
}

fn flatten_or_move<P, Q>(path: P, dest: Q) -> io::Result<()>
where
	P: AsRef<Path>,
	Q: AsRef<Path>,
{
	let path = path.as_ref();
	let dest = dest.as_ref();
	if dest.is_file() {
		return Err(Error::new(
			ErrorKind::InvalidInput,
			"Path is not a directory",
		));
	}
	if path.is_file() {
		if path.parent().map(|dir| dir == dest).unwrap_or(false) {
			return Ok(());
		}
		let mut to = dest.to_path_buf();
		to.push(path.file_name().unwrap());
		fs::rename(path, to)?;
	} else {
		for entry in fs::read_dir(path)? {
			flatten_or_move(entry?.path(), dest)?;
		}
		if path != dest {
			fs::remove_dir(path)?;
		}
	}
	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	const TEST_DIR: &str = "./test_dir";

	fn make_test_dir() -> io::Result<()> {
		fs::create_dir(TEST_DIR)?;
		fs::create_dir("./test_dir/test_subdir")?;
		fs::write("./test_dir/test_file.txt", "test")?;
		fs::write("./test_dir/test_subdir/test_subfile.txt", "test")?;
		Ok(())
	}

	fn remove_test_dir() -> io::Result<()> {
		fs::remove_dir_all(TEST_DIR)
	}

	#[test]
	fn test_flatten() {
		make_test_dir().unwrap();
		flatten(TEST_DIR).unwrap();
		let files: Vec<_> = fs::read_dir(TEST_DIR)
			.unwrap()
			.map(|entry| entry.unwrap().path().file_name().unwrap().to_owned())
			.collect();
		assert_eq!(files, ["test_file.txt", "test_subfile.txt"]);
		remove_test_dir().unwrap();
	}
}
