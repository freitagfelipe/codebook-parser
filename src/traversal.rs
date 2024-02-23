use std::{fs, mem, path::PathBuf};

pub fn get_all_files_path_in_dir(dir_path: &PathBuf) -> Vec<PathBuf> {
    let mut files_path = vec![];

    for entry in fs::read_dir(dir_path).unwrap() {
        let entry_path = entry.unwrap().path();

        if entry_path.is_dir() {
            let mut files_path_result = self::get_all_files_path_in_dir(&entry_path);

            if files_path.len() < files_path_result.len() {
                mem::swap(&mut files_path, &mut files_path_result);
            }

            for file_path in files_path_result {
                files_path.push(file_path);
            }
        }

        if entry_path.is_file() {
            files_path.push(entry_path);
        }
    }

    files_path
}
