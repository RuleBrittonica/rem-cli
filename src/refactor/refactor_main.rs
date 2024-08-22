use clap::error;
use log::{
    info,
    error
};

use crate::refactor::{
    borrow::borrow,
    dump_methods::dump_method_call_types,
    non_local_controller::non_local_controller,
    repair_lifetime::repair_lifetime,
};

pub fn invoke_extraction(

) -> bool {
    todo!()
}

fn extract_function(

    dump: bool,
) -> bool {

    // Set everything up the same way Sewen did

    if dump {
        if dump_method_call_types() {
            // Log successful dump
            info!("Dumped call types completed successfully");
            true
        }
        else {
            // Log unsuccessful dump
            error!("Dumped call types NOT completed");
            false
        }
    }

    else {
        if dump_method_call_types() {
            // Log successful dump
            info!("Dumped call types completed successfully");

            if non_local_controller() {
                // Log successful ontroller
                info!("Controller completed successfully");

                if borrow() {
                    // Log successful borrow
                    info!("Borrow completed succesfully");

                    if repair_lifetime() {
                        // Log successful repair of lifetimes
                        info!("Repairer completed successfully");
                        true // All stages complete
                    }

                    else {
                        // Log unsuccessful repair of lifetimes
                        error!("Repairer NOT completed");
                        false
                    }
                }

                else {
                    // Log unsuccessful borrow
                    error!("Borrow NOT completed - Repairer will not be executed");
                    false
                }
            }

            else {
                // Log unsuccessful controller
                error!("Controller NOT completed - Borrower and Repairer will not be executed");
                false
            }
        }

        else {
            // Log unsuccessful dump
            error!("Dump NOT Completed - Controller, Borrower and Repairer will not be completed");
            false
        }
    }
}

