use structopt::StructOpt;
use crate::math::factorial;
use crate::consoletools::{triangle, hangman};
use std::time::SystemTime;


pub mod math;
pub mod consoletools;
pub mod database;

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
    },

    Hangman {
        word_length: i32,
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
        let sys_time = SystemTime::now();

        let result = match self {
            RustLearnConfig::Factorial { number}  => factorial(number),
            RustLearnConfig::Triangle { number, invert}  => triangle(number, invert),
            RustLearnConfig::Hangman { word_length } => hangman(word_length)
        };
        let new_sys_time = SystemTime::now();

        //Calculate run time
        let difference = new_sys_time.duration_since(sys_time);
        match difference {
            Result::Ok(duration) => println!("Run Time : {} us", duration.as_micros()),
            Result::Err(e) => println!("Run Time Calc Error : {e}")

        }

        result
    }
}
