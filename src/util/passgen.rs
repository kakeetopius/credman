use crate::util::errors::{CMError, CustomError};

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

const MAX_PASSLEN: usize = 255;
const DEFAULT_PASSLEN: usize = 16;

pub fn get_random_pass(passlen: Option<usize>) -> Result<String, CMError> {
    let passlen = passlen.unwrap_or(DEFAULT_PASSLEN);
    if passlen > MAX_PASSLEN {
        return Err(CustomError::new(&format!(
            "Password length provided is above the upper limit of {} characters",
            MAX_PASSLEN
        ))
        .into());
    }
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()";
    let chars_arr: Vec<char> = chars.chars().collect();
    let chars_size = chars.len();

    let mut pass = String::new();
    let mut index_buff: Vec<u8> = vec![0; passlen];

    let mut rng = match ChaCha20Rng::try_from_os_rng() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return Err(CustomError::new("Error generating Password").into());
        }
    };
    rng.fill_bytes(&mut index_buff);

    for i in 0..passlen {
        let mut index: usize = index_buff[i] as usize;
        index = index % chars_size;
        let passchar = &chars_arr[index];

        pass.push(*passchar);
    }

    Ok(pass)
}
