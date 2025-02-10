use std::path::PathBuf;

pub fn repairer(
    file_path: &PathBuf,
    new_file_path: &PathBuf,
    fn_name: &String,
    repairer: &u8,
    verbose: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!("Implement repairer command");
}

pub fn repairer_cargo(
    src_path: &PathBuf,
    manifest_path: &PathBuf,
    fn_name: &String,
    repairer: &u8,
    verbose: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!("Implement repairer-cargo command");
}