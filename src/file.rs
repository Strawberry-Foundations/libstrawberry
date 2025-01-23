use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Reads the entire contents of a file into a string
///
/// # Examples
///
/// ```
/// use stblib::file::read_file;
/// let content = read_file("example.txt")?;
/// ```
///
/// # Errors
///
/// Returns `io::Error` if:
/// - The file doesn't exist
/// - The process lacks permissions
/// - Any other I/O error occurs
pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let mut content = String::new();
    File::open(file_path)?.read_to_string(&mut content)?;
    Ok(content)
}

/// Writes a string to a file
///
/// # Examples
///
/// ```
/// use stblib::file::write_to_file;
/// write_to_file("example.txt", "Hello World!")?;
/// ```
///
/// # Errors
///
/// Returns `io::Error` if:
/// - The file cannot be created
/// - The process lacks permissions
/// - Any other I/O error occurs
pub fn write_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    File::create(file_path)?.write_all(content.as_bytes())
}

#[derive(Debug, Clone)]
pub struct FileStruct {
    file_path: String,
}

impl FileStruct {
    /// Creates a new FileStruct instance that represents a file on the filesystem
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file that should be handled
    ///
    /// # Example
    ///
    /// ```
    /// use stblib::file::FileStruct;
    /// let file = FileStruct::new("example.txt");
    /// ```
    #[must_use]
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().into_owned(),
        }
    }

    /// Reads the entire contents of the file into a string
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The file contents as a string
    /// * `Err(io::Error)` - If the file cannot be read
    ///
    /// # Example
    ///
    /// ```
    /// use stblib::file::FileStruct;
    /// let file = FileStruct::new("example.txt");
    /// let contents = file.read()?;
    /// ```
    pub fn read(&self) -> io::Result<String> {
        read_file(&self.file_path)
    }

    /// Writes content to the file, creating it if it doesn't exist
    ///
    /// # Arguments
    ///
    /// * `content` - The string content to write to the file
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the write was successful
    /// * `Err(io::Error)` - If the file cannot be written
    ///
    /// # Example
    ///
    /// ```
    /// use stblib::file::FileStruct;
    /// let file = FileStruct::new("example.txt");
    /// file.write("Hello, World!")?;
    /// ```
    pub fn write(&self, content: &str) -> io::Result<()> {
        write_to_file(&self.file_path, content)
    }
}