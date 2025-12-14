use std::ffi::OsString;
use crate::template::{all_days, run_multi::run_multi};

pub fn handle(is_release: bool, extra_args:&[OsString]) {
    run_multi(&all_days().collect(), is_release, false, extra_args);
}
