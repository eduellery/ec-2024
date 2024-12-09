use std::fs;
use std::io::{BufRead, BufReader, Error};

pub fn read_file(file_name: &str) -> Result<String, Error> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

pub fn read_file_lines(file_name: &str) -> Result<Vec<String>, Error> {
    let file = fs::File::open(file_name)?;
    let buf = BufReader::new(file);
    let lines = buf.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_file("res/content.txt");
        assert_eq!(
            result.expect("Failed to read test file").trim(),
            "This file contains this content"
        );
    }
}
