use std::fs::{rename, File};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct FileData {
    oldName: String,
    newName: String,
}

impl FileData {
    fn is_empty(&self) -> bool {
        self.oldName.trim().is_empty() || self.newName.trim().is_empty()
    }

    fn rename(self) -> bool {
        match rename(self.oldName, self.newName) {
            Ok(_) => true,
            Err(err) => {
                println!("Failed with error: {:#?}", err);
                false
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:#?}", get_filenames_to_rename_from_file("test.txt"));
}

fn get_filenames_to_rename_from_file(filename: &str) -> Option<Vec<FileData>> {
    let mut files: Vec<FileData> = Vec::new();

    let file = match File::open(filename) {
        Ok(f) => f,
        _ => return None,
    };

    let reader = BufReader::new(file);

    for line in reader
        .lines()
        .filter_map(|x| x.ok())
        .filter(|st| !st.is_empty())
    {
        let seperator = " --> ";
        let possible_arrow_pos = line.find(seperator);
        let fileData = {
            if let Some(pos) = possible_arrow_pos {
                Some(FileData {
                    oldName: String::from(line[..pos].trim()),
                    newName: String::from(line[(pos + seperator.len())..].trim()),
                })
            } else {
                None
            }
        };

        if let Some(fd) = fileData {
            if !fd.is_empty() {
                files.push(fd);
            }
        }
    }

    if files.len() > 0 {
        Some(files)
    } else {
        None
    }
}
