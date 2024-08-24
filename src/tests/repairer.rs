use clap::ValueEnum;
use rem_repairer::{
    common::{
        RepairResult,
        RepairSystem,
    },
    repair_lifetime_loosest_bound_first,
    repair_lifetime_simple,
    repair_lifetime_tightest_bound_first,
    repair_rustfix,
};
use colored::Colorize;
use std::{
    iter::zip,
    time::SystemTime,
    io,
    path::Path,

};
use crate::tests::utils::{
    cleanup_new_files,
    list_files_in_dir,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RepairerType {
    Simple,
    LoosestBoundsFirst,
    TightestBoundsFirst,
}

pub fn test() -> Result<(), io::Error> {
    let file_names: Vec<&str> = vec![
        "borrow",
        "in_out_lifetimes",
        "lifetime_bounds",
        "in_out_lifetimes_original_extract",
        "lifetime_bounds_not_enough_annotations",
        "in_out_lifetimes_wide_bounds",
        "bounds_elider",
    ];
    let function_sigs: Vec<(&str, &str)> = vec![
        ("", ""),
        (
            "bar_extracted",
            "fn bar_extracted(x_ref: &i32, z: &i32, y: &i32) -> &i32",
        ),
        ("bar_extracted", "fn bar_extracted(p: &mut & i32, x: & i32)"),
        ("", ""),
        ("", ""),
        (
            "bar_extracted",
            "fn bar_extracted<'a, 'b, 'c>(x_ref: &'a i32, z: &'b i32, y: &'c i32) -> &'a i32 {",
        ),
        ("bar", ""),
    ];
    let repair_systems: Vec<&dyn RepairSystem> = vec![
        &repair_lifetime_simple::Repairer {},
        &repair_rustfix::Repairer {},
        &repair_lifetime_tightest_bound_first::Repairer {},
        &repair_lifetime_loosest_bound_first::Repairer {},
    ];

    let current_dir = Path::new("./");
    let initial_files = list_files_in_dir(current_dir)?;

    for (file_name, (fn_name, _)) in zip(file_names, function_sigs) {
        for repair_system in repair_systems.iter() {
            let new_file_name = format!("./src_tests/repairer/output/{}{}.rs", file_name, repair_system.name());
            let file_name = format!("./src_tests/repairer/input/{}.rs", file_name);
            print_repair_stat(
                repair_system,
                file_name.as_str(),
                new_file_name.as_str(),
                fn_name,
            )?;
        }
        println!("------------------------------------------------------------------");
    }

    cleanup_new_files(initial_files, current_dir)?;

    Ok(())
}
// fn print_repair_stat_project(
//     repair_system: &&dyn RepairSystem,
//     src_path: &str,
//     manifest_path: &str,
//     fn_name: &str,
// ) -> bool {
//     println!("\n\n{}: {}", src_path, fn_name);
//     let now = SystemTime::now();
//     let RepairResult { success, .. } =
//         repair_system.repair_project(src_path, manifest_path, fn_name);
//     let time_elapsed = now.elapsed().unwrap();
//     println!(
//         "{}: {} refactored {} in {:#?}",
//         (if success {
//             format!("PASSED").green()
//         } else {
//             format!("FAILED").red()
//         }),
//         repair_system.name(),
//         src_path,
//         time_elapsed
//     );
//     success
// }

fn print_repair_stat(
    repair_system: &&dyn RepairSystem,
    file_name: &str,
    new_file_name: &str,
    fn_name: &str,
) -> Result<bool, io::Error> {
    println!("\n\n{}: {}", file_name, fn_name);
    let now = SystemTime::now();
    let RepairResult { success, .. } =
        repair_system.repair_function(file_name, new_file_name, fn_name);
    let time_elapsed = now.elapsed().unwrap();
    println!(
        "{}: {} refactored {} in {:#?}",
        (if success {
            format!("PASSED").green()
        } else {
            format!("FAILED").red()
        }),
        repair_system.name(),
        file_name,
        time_elapsed
    );
    Ok(success)
}