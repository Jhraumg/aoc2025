use std::ffi::OsString;
use std::process::{Command, Stdio};

use crate::template::Day;

pub fn handle(day: Day, release: bool, dhat: bool, submit_part: Option<u8>, extra_args: &[OsString]) {
    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), day.to_string()];

    if dhat {
        cmd_args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ]);
    } else if release {
        cmd_args.push("--release".to_string());
    }
    for arg in extra_args.iter().filter_map(|a| a.to_str()) {
        cmd_args.push(arg.to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string());
    }

    eprintln!("about to run cargo {}", &cmd_args.join(" "));

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
