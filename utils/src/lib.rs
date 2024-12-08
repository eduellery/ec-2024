use std::fs;
use std::io::Error;

pub fn read_file_to_string(file_name: &str) -> Result<String, Error> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_file_to_string("res/content.txt");
        assert_eq!(result.expect("Failed to read test file").trim(), "This file contains this content");
    }
}
