use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct Problem {
    pub data: String,
}

impl Problem {
    pub fn new(file_name: &str) -> Self {
        let path = PathBuf::from(file_name);
        let data = Self::read_file_to_string(&path);
        Self { data }
    }

    fn read_file_to_string(file: &Path) -> String {
        let mut s = String::new();
        let mut file = File::open(file).unwrap();
        file.read_to_string(&mut s).unwrap();
        s
    }
}
