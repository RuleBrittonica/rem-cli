use std::path::PathBuf;
use std::process::exit;
use log::{
    info,
    error
};
use colored::Colorize;
use crate::local_llbc_conversion;
use crate::local_coq_conversion;
use crate::local_coq_verification;
use crate::Extract;
use crate::local_extract_method;
use crate::replace_text_in_file;
use crate::delete_files;
use crate::backup_file;
use crate::utils::delete_backup;

pub fn run_short(
    file_path: &PathBuf,
    new_fn_name: &String,
    start_index: &usize,
    end_index: &usize,
    verbose: &bool,
    cleanup: &bool,
    charon_path: &Option<PathBuf>,
    aeneas_path: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if *verbose {
        info!("Running RunShort in verbose mode");
        info!("File Path: {:?}", file_path);
        info!("New Function Name: {:?}", new_fn_name);
        info!("Start Index: {:?}", start_index);
        info!("End Index: {:?}", end_index);
    }

    // 1. Create the backup
    let backup_path = backup_file(file_path.clone());
    // This is now done in the main function, so we can skip this step.

    // 2. Create the original llbc file
    // We need to determine where to save the output file. This will
    // depend on if the user has specified a project path or a file
    // path. If the user has specified a dir, assume that the output
    // should go in there under dir/dir_name.llbc
    // If the user has specified a file, assume that the output should
    // go in the same directory as the file, with the same name as the
    // file, but with a .llbc extension.

    let out_path: PathBuf = match file_path.is_dir() {
        true => {
            let mut out_path = file_path.clone();
            out_path.push(file_path.file_name().unwrap());
            out_path.set_extension("llbc");
            out_path
        },
        false => {
            let mut out_path = file_path.clone();
            out_path.set_extension("llbc");
            out_path
        }
    };

    info!("Output path: {:?}", out_path);

    let original_llbc_path: PathBuf = match local_llbc_conversion(
        file_path,
        &out_path,
        charon_path
    ) {
        Ok(output_path) => {
            // Verify that there is a file at the output path.
            if output_path.exists() {
                info!("Conversion to LLBC succeeded for project: {:?}", file_path);
                info!("LLBC file saved at: {:?}", output_path);
                output_path
            } else {
                error!("Conversion to LLBC failed: No file found at output path: {:?}", output_path);
                return Err("Conversion to LLBC failed: No file found at output path".into());
            }
        },
        Err(e) => {
            error!("Conversion to LLBC failed: {}", e);
            return Err(e);
        }
    };

    // Step 3: Extract the method.
    // This is similar to what we do in REMCommands::Extract.
    let start_idx: u32 = *start_index as u32;
    let end_idx: u32 = *end_index as u32;
    let extraction_input: Extract = Extract::new(
        file_path.to_path_buf(),
        None, // output code not yet populated
        new_fn_name.to_string(),
        start_idx,
        end_idx,
        None, // caller_fn_name not yet populated
    );

    let (output_code, caller_fn_name) = match local_extract_method(extraction_input) {
        Ok(result) => {
            (
                result.get_output_code().unwrap(),
                result.get_caller_fn_name().unwrap(),
            )
        },
        Err(e) => {
            error!("Extraction failed: {:?}", e);
            return Err(e);
        }
    };

    info!("Extraction succeeded. New Code:\n{}", output_code);
    info!("Caller Function Name: {}", caller_fn_name);

    // 4. Write the new code to the original file
    let refactored_file_path = match replace_text_in_file(file_path, &output_code) {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to write new code to file: {:?}", e);
            return Err(e);
        }
    };

    // 5. Create the new llbc file . Same as step 2. The file name will
    //    end up being the same as original_llbc_path, but with a _ref
    //    suffix.
    let mut new_llbc_path: PathBuf = original_llbc_path.clone();
    let stem: &str = new_llbc_path.file_stem().unwrap().to_str().unwrap();
    let ext: &str = new_llbc_path.extension().unwrap().to_str().unwrap();
    new_llbc_path.set_file_name(format!("{}_ref.{}", stem, ext));

    let new_llbc_path: PathBuf = match local_llbc_conversion(
        &refactored_file_path,
        &new_llbc_path,
        charon_path,
    ) {
        Ok(output_path) => {
            // Verify that there is a file at the output path.
            if output_path.exists() {
                info!("Conversion to LLBC succeeded for project: {:?}", file_path);
                info!("LLBC file saved at: {:?}", output_path);
                output_path
            } else {
                error!("Conversion to LLBC failed: No file found at output path: {:?}", output_path);
                return Err("Conversion to LLBC failed: No file found at output path".into());
            }
        },
        Err(e) => {
            error!("Conversion to LLBC failed: {}", e);
            return Err(e);
        }
    };

    // 6. Convert the original and new llbc files to CoQ
    info!("Original LLBC: {:?}", original_llbc_path);
    info!("New LLBC: {:?}", new_llbc_path);
    let (original_coq_path, refactored_coq_path) = match local_coq_conversion(
        &original_llbc_path,
        &new_llbc_path,
        &None,
        aeneas_path,
    ) {
        Ok((original_coq_path, refactored_coq_path)) => {
            info!("Conversion to CoQ succeeded for project: {:?}", file_path);
            info!("Original CoQ file saved at: {:?}", original_coq_path);
            info!("Refactored CoQ file saved at: {:?}", refactored_coq_path);
            (original_coq_path, refactored_coq_path)
        },
        Err(e) => {
            error!("Conversion to CoQ failed: {}", e);
            return Err(e);
        }
    };

    // 7. Verify the original and new CoQ files
    let top_level_function: String = caller_fn_name;
    let (coq_project_path, equiv_check_path, primitives_path, result) = match local_coq_verification(
        &original_coq_path,
        &refactored_coq_path,
        &top_level_function,
    ) {
        Ok((coq_project_path, equiv_check_path, primitives_path, result)) => {
            info!("Verification succeeded for project: {:?}", file_path);
            info!("CoQ Project file saved at: {:?}", coq_project_path);
            info!("EquivCheck file saved at: {:?}", equiv_check_path);
            info!("Primitives file saved at: {:?}", primitives_path);
            (coq_project_path, equiv_check_path, primitives_path, result)
        },
        Err(e) => {
            error!("Verification failed: {}", e);
            return Err(e);
        }
    };

    if result {
        info!("Verification succeeded for project: {:?}", file_path);
        info!("Original code is garunteed to be equivalent to the refactored code!")
    } else {
        error!("Verification failed for project: {:?}", file_path);
        return Err("Verification failed".into());
    }

    // 8. Ensure success and cleanup.
    // Delete the backup, original llbc, and new llbc files.
    // Delete the coqproject, EquivCheck, and Primitives files, along
    // with the original and refactored CoQ files.
    // If any of these steps fail, log the error and exit with an error
    // code.

    if *cleanup {
        info!("Cleaning up after RunShort");
        info!("Deletingoriginal LLBC: {}, and new LLBC files: {}",
            original_llbc_path.to_str().unwrap(),
            new_llbc_path.to_str().unwrap(),
        );
        let files_to_delete: Vec<PathBuf> = vec![
            original_llbc_path,
            new_llbc_path,
            coq_project_path,
            equiv_check_path,
            primitives_path,
            original_coq_path,
            refactored_coq_path,
        ];

        delete_files(&files_to_delete)?;

        // Attempt to delete the backup
        if let Some(backup_path_real) = backup_path {
            if let Err(e) = delete_backup(backup_path_real) {
                error!("Failed to delete backup: {:?}", e);
                exit(1);
            } else {
                info!("Backup deleted successfully");
            }
        }
    }

    // 9. Print out the results
    info!("RunShort completed successfully");
    eprintln!("RESULT: {}", {
        if result {
            "SUCCESS".green()
        } else {
            "FAILURE".red()
        }
    });

    Ok(())
}