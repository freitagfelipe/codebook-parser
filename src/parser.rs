use std::{fs::File, io::Read, path::PathBuf};

fn get_algorithms_blocks(path: &PathBuf) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut file_content = String::new();

    file.read_to_string(&mut file_content).unwrap();

    let mut code_snippets = Vec::new();

    for entry in file_content.split("```") {
        if entry.starts_with("cpp") {
            let code_snippet = entry.strip_prefix("cpp").unwrap();

            code_snippets.push(code_snippet.to_owned());
        }
    }

    code_snippets
}
