use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file_in_lines(path: &str) -> io::Result<std::io::Lines<BufReader<File>>> {
    let file = File::open(path).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("File {:?} should be there", path),
        )
    })?;
    return Ok(io::BufReader::new(file).lines());
}
