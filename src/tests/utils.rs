use std::{
    fs, io, path::Path, process::Command
};

pub fn list_files_in_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(files)
}

pub fn cleanup_new_files(initial_files: Vec<String>, current_dir: &Path) -> io::Result<()> {
    let final_files = list_files_in_dir(current_dir)?;

    // Determine which files are new
    let new_files: Vec<String> = final_files
        .into_iter()
        .filter(|file| !initial_files.contains(file))
        .collect();

    // Delete new files
    for file in new_files {
        let file_path = current_dir.join(&file);
        fs::remove_file(file_path)?;
    }

    Ok(())
}

pub fn compile_file(file_name: &str, args: &Vec<&str>) -> Command {
    let mut compile = Command::new("rustc");
    for arg in args {
        compile.arg(arg);
    }
    compile.arg(file_name);
    compile
}