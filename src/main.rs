extern crate clap;
extern crate sempfind;

use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("Semperos Find")
        .version("0.1.0")
        .author("Daniel Gregoire <daniel.l.gregoire@gmail.com>")
        .about("Find files and things by name")
        .arg(
            Arg::with_name("directory_name")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY_NAME")
                .help("Directory's name (case-insensitive substring)")
        )
        .arg(
            Arg::with_name("file_name")
                .short("f")
                .long("file")
                .value_name("FILE_NAME")
                .help("File's name (case-insensitive substring)"),
        )
        .get_matches();

    if let Some(file_name) = matches.value_of("file_name") {
        if let Err(e) = sempfind::find_file(file_name) {
            eprintln!("[ERROR] Failed to find {}, details: {:?}", file_name, e);
            process::exit(1);
        } else {
            process::exit(0);
        }
    }

    if let Some(directory_name) = matches.value_of("directory_name") {
        if let Err(e) = sempfind::find_directory(directory_name) {
            eprintln!("[ERROR] Failed to find {}, details: {:?}", directory_name, e);
            process::exit(1);
        } else {
            process::exit(0);
        }
    }
    println!("{}", matches.usage());
}
