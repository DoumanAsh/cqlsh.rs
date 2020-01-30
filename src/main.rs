#![no_main]
mod cli;
mod auth;
mod shell;

use std::io::BufRead;

use auth::Auth;

const FAIL_CODE: isize = 1;
const DEFAULT_HOST: &str = "127.0.0.1:9042";

#[no_mangle]
unsafe extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let mut result = 0;
    let args = c_ffi::Args::new(argc, argv).expect("To get function arguments");

    let args = match cli::parse(args.into_iter()) {
        Ok(args) => args,
        Err(code) => std::process::exit(code),
    };

    if args.version {
        println!(concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION")));
        return 0
    }

    let auth = Auth::from_creds(args.username, args.password);
    let host = args.host.unwrap_or_else(|| DEFAULT_HOST.to_owned());
    let shell = match shell::Shell::new(&host, auth) {
        Ok(shell) => shell,
        Err(error) => {
            eprintln!("Unable to connect to {}. Error: {}", host, error);
            return FAIL_CODE;
        }
    };

    if let Some(statement) = args.execute {
        if !shell.execute(&statement) {
            result = FAIL_CODE
        }
    } else if let Some(file) = args.file {
        let file = match std::fs::File::open(&file) {
            Ok(file) => std::io::BufReader::new(file),
            Err(error) => {
                eprintln!("{}: No such file. Error {}", file, error);
                return FAIL_CODE;
            }
        };

        for line in file.lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    eprintln!("Error reading file: {}", error);
                    return FAIL_CODE;
                }
            };

            if !shell.execute(&line) {
                result = FAIL_CODE
            }
        }

    } else if !shell.interactive() {
        result = FAIL_CODE
    }

    result
}
