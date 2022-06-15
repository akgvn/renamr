use std::env;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct FileData {
    old_name: String,
    new_name: String,
}

impl FileData {
    fn is_empty(&self) -> bool {
        self.old_name.trim().is_empty() || self.new_name.trim().is_empty()
    }

    fn rename(self) {
        match rename(&self.old_name, &self.new_name) {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "Renaming \"{}\" failed with error: {:?}",
                    self.old_name,
                    err.kind()
                );
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect::<_>();
    if args.len() < 2 {
        println!("No filename argument given!");
        return;
    }

    let filename = args[1].to_owned();

    let files = get_filenames_to_rename_from_file(&filename).unwrap();
    for file in files {
        file.rename();
    }
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
        let file_data = {
            if let Some(pos) = possible_arrow_pos {
                Some(FileData {
                    old_name: String::from(line[..pos].trim()),
                    new_name: String::from(line[(pos + seperator.len())..].trim()),
                })
            } else {
                None
            }
        };

        if let Some(fd) = file_data {
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
