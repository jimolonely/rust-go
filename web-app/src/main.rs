use std::fs::File;
use std::io::Write;

fn main() {
    match File::create("foo.txt") {
        Ok(..) => println!("File created"),
        Err(..) => println!("Error")
    }
}

fn log_something(filename: &'static str, string: &'static [u8; 12])
{
    let mut f = File::create(filename)?;
    f.write_all(string)?
}