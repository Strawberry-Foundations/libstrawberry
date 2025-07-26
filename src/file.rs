use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Reads the entire contents of a file into a string
///
/// # Examples
///
/// ```
/// use libstrawberry::file::read_file;
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
/// use libstrawberry::file::write_to_file;
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
pub struct PersistentFile {
    file_path: String,
}

impl PersistentFile {
    /// Creates a new `PersistentFile` instance that represents a file on the filesystem
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file that should be handled
    ///
    /// # Example
    ///
    /// ```
    /// use libstrawberry::file::PersistentFile;
    /// let file = PersistentFile::new("example.txt");
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
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if:
    /// * The file does not exist
    /// * The process lacks permissions to read the file
    /// * The file cannot be read due to system I/O errors
    ///
    /// # Example
    ///
    /// ```
    /// use std::io;
    /// use libstrawberry::file::PersistentFile;
    ///
    /// fn read_example() -> io::Result<()> {
    ///     let file = PersistentFile::new("example.txt");
    ///     let contents = file.read()?;
    ///     println!("File contents: {}", contents);
    ///     Ok(())
    /// }
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
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if:
    /// * The file cannot be created
    /// * The process lacks permissions to write to the file
    /// * The file cannot be written due to system I/O errors
    ///
    /// # Example
    ///
    /// ```
    /// use std::io;
    /// use libstrawberry::file::PersistentFile;
    ///
    /// fn write_example() -> io::Result<()> {
    ///     let file = PersistentFile::new("example.txt");
    ///     file.write("Hello, World!")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn write(&self, content: &str) -> io::Result<()> {
        write_to_file(&self.file_path, content)
    }
}
