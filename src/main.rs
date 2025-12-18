use advent_of_code::template::commands::{all, download, read, scaffold, solve, time};
use args::{AppArguments, parse};

#[cfg(feature = "today")]
use advent_of_code::template::Day;
#[cfg(feature = "today")]
use std::process;

mod args {
    use advent_of_code::template::Day;
    use std::ffi::OsString;
    use std::process;

    pub enum AppArguments {
        Download {
            day: Day,
        },
        Read {
            day: Day,
        },
        Scaffold {
            day: Day,
            download: bool,
            overwrite: bool,
        },
        Solve {
            day: Day,
            release: bool,
            dhat: bool,
            submit: Option<u8>,
            extra_args: Vec<OsString>,
        },
        All {
            release: bool,
            extra_args: Vec<OsString>,
        },
        Time {
            all: bool,
            day: Option<Day>,
            store: bool,
            extra_args: Vec<OsString>,
        },
        #[cfg(feature = "today")]
        Today,
    }

    pub fn parse() -> Result<AppArguments, Box<dyn std::error::Error>> {
        let mut os_args: Vec<OsString> = std::env::args_os().collect();
        os_args.remove(0); // remove the executable path.

        let main_args: Vec<OsString> = os_args
            .iter()
            .take_while(|arg| *arg != "--")
            .cloned()
            .collect();
        let extra_args: Vec<OsString> = os_args
            .into_iter()
            .skip_while(|arg| arg != "--")
            .skip(1)
            .collect();

        let mut args = pico_args::Arguments::from_vec(main_args);

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => AppArguments::All {
                release: args.contains("--release"),
                extra_args,
            },
            Some("time") => {
                let all = args.contains("--all");
                let store = args.contains("--store");

                AppArguments::Time {
                    all,
                    day: args.opt_free_from_str()?,
                    store,
                    extra_args,
                }
            }
            Some("download") => AppArguments::Download {
                day: args.free_from_str()?,
            },
            Some("read") => AppArguments::Read {
                day: args.free_from_str()?,
            },
            Some("scaffold") => AppArguments::Scaffold {
                day: args.free_from_str()?,
                download: args.contains("--download"),
                overwrite: args.contains("--overwrite"),
            },
            Some("solve") => AppArguments::Solve {
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                dhat: args.contains("--dhat"),
                extra_args,
            },
            #[cfg(feature = "today")]
            Some("today") => AppArguments::Today,
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArguments::All {
                release,
                extra_args,
            } => all::handle(release, &extra_args),
            AppArguments::Time {
                day,
                all,
                store,
                extra_args,
            } => time::handle(day, all, store, &extra_args),
            AppArguments::Download { day } => download::handle(day),
            AppArguments::Read { day } => read::handle(day),
            AppArguments::Scaffold {
                day,
                download,
                overwrite,
            } => {
                scaffold::handle(day, overwrite);
                if download {
                    download::handle(day);
                }
            }
            AppArguments::Solve {
                day,
                release,
                dhat,
                submit,
                extra_args,
            } => solve::handle(day, release, dhat, submit, &extra_args),
            #[cfg(feature = "today")]
            AppArguments::Today => {
                match Day::today() {
                    Some(day) => {
                        scaffold::handle(day, false);
                        download::handle(day);
                        read::handle(day)
                    }
                    None => {
                        eprintln!(
                            "`today` command can only be run between the 1st and \
                            the 25th of december. Please use `scaffold` with a specific day."
                        );
                        process::exit(1)
                    }
                };
            }
        },
    };
}
