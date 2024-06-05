use std::fs;
use std::path::Path;

#[derive(Debug)]
/// FileProcessor is a struct with `read` as main operation, which opens a .net (or other simple text based file, such as .txt)
/// and stores it in the **content** property as a **String**.
pub struct FileProcessor {
    content: String,
}

impl FileProcessor {
    /// Opens a file and read it's content, storing as a String on the **content** property,
    /// trimming the empty lines in the end if necessary.
    ///
    /// # Arguments
    ///
    /// * `filename (&str)` - full path, relative to root of project, to the data source.
    /// * `verbose (bool)` - print informations usefull for debugging.
    ///
    /// # Returns
    ///
    /// A `Result` with either the resulting FileProcessor, with the content of the file in the
    /// `content` property, or a &str with the error.
    pub fn read(filename: &str, verbose: bool) -> Result<Self, &'static str> {
        if verbose {
            println!("Reading file in path: {}", filename);
        }

        // this to access files in any place on the root of the project
        let mut file = "";
        let mut path = "";
        if filename.contains("/") {
            if let Some((remaining, data_file)) = filename.rsplit_once('/') {
                file = data_file;
                path = remaining;
            }
        } else {
            file = filename;
        }

        let path: std::path::PathBuf = Path::new(path).join(file);

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

    pub fn content(&self) -> String {
        self.content.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        let file_path = "data/test_test.net";

        // preparing test file
        let test_content = "First line\nSecond line\n\n\n";
        let filename = "test_test.net";
        let test_path = Path::new("data").join(filename);
        fs::write(&test_path, test_content).expect("Failed to write the test file");

        let result = FileProcessor::read(file_path, false);

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
