use std::fs::File;
use std::io::{Read, Write};

pub struct FileHandler;

pub struct FileStruct {
    pub file_path: String,
}

impl FileHandler {
    pub fn read_file(file_path: &str) -> String {
        let mut file = File::open(file_path).expect("Error while opening file");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("TODO: panic message");

        content
    }

    pub fn write_to_file(file_path: &str, content: &str) {
        let mut file = File::create(file_path).expect("Error while creating or opening file");

        file.write_all(content.as_bytes())
            .expect("Error while writing to file");
    }
}

impl FileStruct {
    pub fn read(&self) -> String {
        let mut file: File = File::open(&self.file_path).expect("Error while opening file");

        let mut content = String::new();

        file.read_to_string(&mut content)
            .expect("Error while reading file");

        content
    }

    pub fn write(&self, content: &str) {
        let mut file = File::create(&self.file_path).expect("Error while creating or opening file");

        file.write_all(content.as_bytes())
            .expect("Error while writing to file");
    }
}
