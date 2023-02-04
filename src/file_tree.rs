use std::{fmt::format, io::Write, fs::create_dir};

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub entries: Vec<File>,
    pub diretory: Vec<Directory>,
}

impl File {
    pub fn new(name: String, data: Vec<u8>) -> File {
        File { name, data }
    }
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name,
            entries: Vec::new(),
            diretory: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.entries.push(file);
    }

    fn add_directory(&mut self, directory: Directory) {
        self.diretory.push(directory);
    }

    pub fn list(&self) {
        println!("Directory: {}", self.name);
        for file in &self.entries {
            println!("File: {}", file.name);
        }
        for directory in &self.diretory {
            directory.list();
        }
    }

    fn find(&self, name: &str) -> Option<&File> {
        for file in &self.entries {
            if file.name == name {
                return Some(file);
            }
        }
        for directory in &self.diretory {
            match directory.find(name) {
                Some(file) => return Some(file),
                None => (),
            }
        }
        None
    }

    pub fn map_directory(&mut self, dir_location: String) -> Directory {
        let mut new_dir = Directory::new(dir_location);
        let directory_content = std::fs::read_dir(&new_dir.name)
            .expect("Failed to read directory are you sure it exists?");

        for content in directory_content {
            let content = content.expect("Failed to read directory content");
            let content_path = content.path();

            if content_path.is_dir() {
                let mut new_dir = Directory::new(content_path.to_str().unwrap().to_string());
                new_dir.map_directory(content_path.to_str().unwrap().to_string());
                self.add_directory(new_dir);
            }

            if content_path.is_file() {
                self.add_file(File::new(
                    content_path.to_str().unwrap().to_string(),
                    std::fs::read(content_path).expect("Failed to read file"),
                ));
            }
        }

        new_dir
    }


    pub fn write_tree_to_file(&self) {
        let mut file = std::fs::File::create("file_tree.txt").expect("Failed to create file");
        for file_data in &self.entries {
            file.write_all(file_data.name.as_bytes())
                .expect("Failed to write to file");
            file.write_all(&file_data.data.as_slice());
        }
    }

    pub fn move_dir(&self, dir_location: String) {
        if !std::path::Path::new(&dir_location).exists() {
            std::fs::create_dir(&dir_location);
        }

        for content in &self.entries {
            for directory in &self.diretory {
                if !std::path::Path::new(&format!("{dir_location}/{}", directory.name)).exists() {
                    std::fs::create_dir(format!("{dir_location}/{}", directory.name));
                }
                directory.move_files(dir_location.clone());
            }
            for file in &self.entries {
                let file_name = file.name.split("/").last().unwrap();
                let mut file_create = std::fs::File::create(format!("{dir_location}/{file_name}"))
                    .expect("Failed to create file");
                file_create
                    .write_all(&file.data.as_slice())
                    .expect("Failed to write to file");
            }
        }
    }

    pub fn move_files(&self, dir_location: String) {
        for file in &self.entries {
            println!("File: {}", file.name);
            let mut file_create = std::fs::File::create(format!("{dir_location}/{}", file.name))
                .expect("Failed to create file");
            file_create
                .write_all(&file.data.as_slice())
                .expect("Failed to write to file");
        }
    }
}
