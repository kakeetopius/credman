use crate::util::errors::CustomError;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn get_random_pass() -> Result<String, Box<dyn Error>> {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()";
    let chars_arr: Vec<char> = chars.chars().collect();

    let mut pass = String::new();

    let mut rand_file = File::open("/dev/urandom")?;

    let mut index_buff: [u8; 16] = [0; 16];
    let readbytes = rand_file.read(&mut index_buff)?;

    if readbytes < 16 {
        return Err(Box::new(CustomError::new(
            "Error reading from urandom file",
        )));
    }
    let chars_size = chars.len();

    for i in 0..16 {
        let mut index: usize = index_buff[i] as usize;
        index = index % chars_size;
        let passchar = &chars_arr[index];

        pass.push(*passchar);
    }

    Ok(pass)
}
