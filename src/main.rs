use include_dir::include_dir;
use std::{
    error::Error,
    io::{self, Write},
    time::Duration,
};

const SOURCE_DIR: include_dir::Dir<'_> = include_dir!("./files_nocolor/");

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let argv1 = args
        .nth(1)
        .unwrap_or("40000000".to_owned())
        .parse()
        .unwrap_or(40000000);

    let ghostty = SOURCE_DIR;

    let mut stderr = io::stderr();

    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    loop {
        for ghost in ghostty.files() {
            stderr.write_all(ghost.contents())?;
            stderr.flush()?;
            std::thread::sleep(Duration::from_nanos(argv1));
        }
    }
}
