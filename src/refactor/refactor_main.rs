use crate::refactor::{
    borrow::borrow,
    dump_methods::dump_method_call_types,
    non_local_controller::non_local_controller,
    repair_lifetime::repair_lifetime,
};

pub fn invoke(

) -> bool {
    todo!()
}

fn extract_function(

    dump: bool,
) -> bool {

    if dump {
        if dump_method_call_types() {
            todo!() // Log successful dump
        }
        else {
            todo!() // Log unsuccessful dump
        }
    }

    else {
        if dump_method_call_types() {
            // Log successful dump

            if non_local_controller() {
                // Log successful ontroller

                if borrow() {
                    // Log successful borrow

                    if repair_lifetime() {
                        // Log successful repair of lifetimes

                        todo!()
                    }

                    else {
                        // Log unsuccessful repair of lifetimes
                        todo!()
                    }
                }

                else {
                    // Log unsuccessful borrow
                    todo!()
                }
            }

            else {
                todo!() // Log unsuccessful controller
            }
        }

        else {
            todo!() // Log unsuccessful dump
        }
    }
}

