use structopt::StructOpt;
use crate::factorial::get_factorial;
use crate::consoletools::triangle;

pub mod factorial;
pub mod consoletools;

#[derive(StructOpt)]
#[structopt(about = "A collection of projects for getting familiar with Rust. See https://github.com/whostolemyhat/learning-projects.")]
pub enum RustLearnConfig {
    Factorial {
        number: i32
    },

    Triangle {
        number: i32,
        #[structopt(short, long)]
        invert: bool,
    }

    /*
    Add {
        #[structopt(short)]
        interactive: bool,
        #[structopt(short)]
        patch: bool,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Fetch {
        #[structopt(long)]
        dry_run: bool,
        #[structopt(long)]
        all: bool,
        repository: Option<String>,
    },
    Commit {
        #[structopt(short)]
        message: Option<String>,
        #[structopt(short)]
        all: bool,
    },
    */
}


impl RustLearnConfig {
    pub fn dispatch(self)->Result <(), &'static str>{
        let result = match self {
            RustLearnConfig::Factorial { number}  => get_factorial(number),
            RustLearnConfig::Triangle { number, invert}  => triangle(number, invert),
        };
        result
    }
}
