use std::io;
use std::process::{self, Command};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct FindError {
    io_error: Option<io::Error>,
    from_utf8_error: Option<FromUtf8Error>,
}

enum FindType {
    File,
    Directory,
}

fn find_file_or_directory(find_type: FindType, name_portion: &str) -> Result<(), FindError> {
    let mut cmd = Command::new("find");
    let s = format!("*{}*", name_portion);
    cmd.arg(".")
        .arg("-type")
        .arg(match find_type {
            FindType::File => "f",
            FindType::Directory => "d",
        })
        .arg("-iname")
        .arg(s);
    let status = cmd.status().expect("find failed to return a status code");
    if !status.success() {
        println!("[ERROR] Find failed.");
        process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

pub fn find_file(name_portion: &str) -> Result<(), FindError> {
    find_file_or_directory(FindType::File, name_portion)
}

pub fn find_directory(name_portion: &str) -> Result<(), FindError> {
    find_file_or_directory(FindType::Directory, name_portion)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
