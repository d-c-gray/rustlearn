use rustlearn::RustLearnConfig;
use structopt::StructOpt;
use std::process;
fn main() {
    let opt = RustLearnConfig::from_args();
    if let Err(e) = opt.dispatch() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
