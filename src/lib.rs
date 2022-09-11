use std::{io::{BufReader, BufRead, Write}, fs::File};

use anyhow::Result;

pub fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in reader.lines().flatten() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use super::*;

    #[test]
    fn find_match() {
        let tmpfile = assert_fs::NamedTempFile::new("temp").unwrap();
        tmpfile.write_str("lorem ipsum\ndolor sit amet").unwrap();

        let tmpfile_file = File::open(tmpfile.path()).unwrap();

        let reader = BufReader::new(tmpfile_file);
        let mut result = Vec::new();
        find_matches(reader, "lorem", &mut result).unwrap();
        assert_eq!(result, b"lorem ipsum\n");
    }
}
