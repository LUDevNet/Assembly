mod sysdiagram;
mod pki;

use pki::{main as pki_main, MainError as PkiError};
use getopts::Options;
use std::env;

#[derive(Debug)]
enum CLIError {
    PKI(PkiError),
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} SUBCOMMAND [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), CLIError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    let subcommand = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    if subcommand == "pki" {
        pki_main(matches.free).map_err(CLIError::PKI)
    } else {
        print_usage(&program, opts);
        Ok(())
    }
}
