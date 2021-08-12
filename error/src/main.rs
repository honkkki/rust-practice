#![feature(with_options)]

use std::fs::File;
use std::io::Write;
use std::io::ErrorKind;

fn main() {
    let file = File::with_options().append(false).write(true).read(true).open("data.txt");
    match file {
        Ok(mut file) => {
            let res = file.write(b"hello");
            match res {
                Ok(len) => {
                    println!("write success: {}bytes.", len)
                }
                Err(e) => println!("write fail: {}", e),
            }
        }

        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let res = File::create("data.txt");
                match res {
                    Ok(mut file) => {
                        let res = file.write("init".as_bytes());
                        match res {
                            Ok(len) => {
                                println!("write success: {}bytes.", len)
                            }
                            Err(e) => println!("write fail: {}", e),
                        }
                    }
                    Err(e) => println!("create file fail: {}", e),
                }
            }

            other_error => panic!("open file error: {:?}", other_error),
        },
    }
    println!("--------------------------------");

    // unwrap
    let mut file = File::with_options().write(true).open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("create file error: {}", error);
            })
        } else {
            panic!("open file error: {}", error);
        }
    });

    let len = file.write(b"test unwrap").expect("write error");
    println!("write bytes: {}", len);
    println!("------------------------------");



}
