use std::fs::File;
use std::io::{Read, Write};

pub struct FileHandler;

pub struct FileStruct {
    pub file_path: String,
}

impl FileHandler {
    /// Simple file handler to read a file
    /// # Panics
    ///
    /// - Will panic if file is not readable
    /// - Will panic if your current user does not have the
    /// required permissions to open the file

    #[must_use]
    pub fn read_file(file_path: &str) -> String {
        let mut file = File::open(file_path).expect("Error while opening file");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Error while reading from file");

        content
    }
    /// Simple file handler to write to a file
    /// # Panics
    ///
    /// - Will panic if file is not writeable
    /// - Will panic if your current user does not have the
    /// required permissions to open the file

    pub fn write_to_file(file_path: &str, content: &str) {
        let mut file = File::create(file_path).expect("Error while creating or opening file");

        file.write_all(content.as_bytes())
            .expect("Error while writing to file");
    }
}

/// Simple file struct for creating an object that can be used to read or write from or to the file
impl FileStruct {
    /// Read data from the file struct
    /// # Panics
    ///
    /// - Will panic if file is not readable
    /// - Will panic if your current user does not have the
    /// required permissions to open the file

    #[must_use]
    pub fn read(&self) -> String {
        let mut file: File = File::open(&self.file_path).expect("Error while opening file");

        let mut content = String::new();

        file.read_to_string(&mut content)
            .expect("Error while reading file");

        content
    }

    /// Write data to the file struct
    /// # Panics
    ///
    /// - Will panic if file is not writeable
    /// - Will panic if your current user does not have the
    /// required permissions to open the file

    pub fn write(&self, content: &str) {
        let mut file = File::create(&self.file_path).expect("Error while creating or opening file");

        file.write_all(content.as_bytes())
            .expect("Error while writing to file");
    }
}
