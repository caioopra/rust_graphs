use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct FileProcessor {
    content: String,
}

// for now, the "data" folder is necessary
// TODO: make generic the folder and filename, throwing a panic in case it doesn't exist
impl FileProcessor {
    pub fn read(filename: &str, verbose: bool) -> Result<Self, &'static str> {
        if verbose {
            println!("Reading file in path: {}", filename);
        }

        let path: std::path::PathBuf = Path::new("data").join(filename);

        // Ok values will be untouched, but if err, will return an Err with the string
        let content = fs::read_to_string(path).map_err(|_| "Couldn't read file from path")?;

        let processed: String = content
            .lines()
            .rev()
            .skip_while(|line| line.trim().is_empty())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");

        if verbose {
            println!("Text from file: {}", processed);
        }

        Ok(FileProcessor { content: processed })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        let filename = "test_test.net";

        // preparing test file
        let test_content = "First line\nSecond line\n\n\n";
        let test_path = Path::new("data").join(filename);
        fs::write(&test_path, test_content).expect("Failed to write the test file");

        let result = FileProcessor::read(filename, false);

        assert!(result.is_ok());

        let processor = result.unwrap();
        assert_eq!(processor.content, "First line\nSecond line");

        fs::remove_file(test_path).expect("Failed to delete the test file");
    }

    #[test]
    fn test_read_file_not_found() {
        let test_filename = "this_file_doesnt_exist.txt";

        let result = FileProcessor::read(test_filename, false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Couldn't read file from path");
    }
}
