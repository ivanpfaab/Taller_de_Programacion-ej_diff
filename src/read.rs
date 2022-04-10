//! Current module is responsible of the handbling of some the read methods
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::process;

/// Opens a File using a valid path
///
/// Given that the path could not exist, you have an amount of trys to open it.
/// In case the user use every attempt available, the program will end, warning
/// the user about the issue.
///
/// This method is used only internally.
///
fn read_file(path: String, trys: u8) -> File {
    match File::open(path) {
        Ok(it) => it,
        Err(_) => {
            println!("There no such file or directory, please try again");
            if trys == 3 {
                println!("\nAre you having problems finding your path?");
                println!("Please find the right one and come back with us whenever you are ready!");
                process::exit(1);
            }
            read_file(read_from_console(), trys + 1)
        }
    }
}

/// Reads the lines of a file given a path passed as parameter.
/// The lines will be returned in a vector of strings.
///
/// # Example
///
/// ```
/// let lines = read::read_file_lines("home/hola.txt");
/// ```
pub fn read_file_lines(path: String) -> Result<Vec<String>, Error> {
    BufReader::new(read_file(path, 1)).lines().collect()
}

/// Reads the input from the console and parse it to a string.
/// Returns the value read or error in case the read_line methods failed.
///
/// # Example
///
/// ```
/// let console_input = read::read_from_console();
/// ```
pub fn read_from_console() -> String {
    let mut file_path = String::new();

    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    file_path.trim().to_string()
}
