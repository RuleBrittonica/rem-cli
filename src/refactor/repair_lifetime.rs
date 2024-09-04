use std::time::Instant;
use log::{
    info,
    error,
};
use rem_repairer::common::{
    RepairSystem,
    RepairResult,
};

pub fn repair_lifetime(
    file_path:     &str,
    new_file_path: &str,
    fn_name:       &str,
    repair_system: &&dyn RepairSystem,

) -> bool {
    let begin: Instant = Instant::now();

    let RepairResult { success, ..} =
        repair_system.repair_function(file_path, new_file_path, fn_name);

    // Handle a failure
    if !success {
        error!("Bad exit value, file will be restored");
    }

    let success_string: &str = if success { "was successful " } else { "failed" };

    info!("Repairer {}, elapsed time in milliseconds: {:?}",
        success_string,
        begin.elapsed().as_millis()
    );

    success

}

pub fn repair_lifetime_rustc(

) -> bool {
    todo!()
}

pub fn repair_lifetime_cargo(

) -> bool {
    todo!()
}