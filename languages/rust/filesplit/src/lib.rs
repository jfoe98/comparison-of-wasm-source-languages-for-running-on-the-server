use std::fs::File;
use std::io::{self, BufRead, prelude::*};
use std::fs::OpenOptions;

#[no_mangle]
pub extern "C" fn filesplit(n: i32) {
    unsafe {
        startup();
    }

    if let Ok(file) = get_file(n) {
        let lines = read_lines(file);

        for line in lines {
            if let Ok(ip) = line {
                let number = ip.parse::<i32>().unwrap();
                write_to_new_file(number).unwrap();
            }
        }
    }

    unsafe {
        finish();
    }
}

fn write_to_new_file(number: i32) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(format!("./{}.txt", number % 10))?;

    writeln!(file, "{}", number)?;
    file.flush()
}

fn get_file(n: i32) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .open(format!("./numbers_{}.txt", n))
}

fn read_lines(file: File) -> io::Lines<io::BufReader<File>> {
    io::BufReader::new(file).lines()
}

#[link(wasm_import_module = "env")]
extern "C" {
    fn startup();
    fn finish();
}