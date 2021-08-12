use std::io;
use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let str = read_data_from_file1().unwrap();
    println!("{}", str);

    let str = read_data_from_file2().unwrap();
    println!("{}", str);
    Ok(())
}

fn read_data_from_file1() -> Result<String, io::Error> {
    let f = File::open("data.txt");
    let mut file = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

fn read_data_from_file2() -> Result<String, io::Error> {
    let mut file = File::open("data.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
