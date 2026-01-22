use crate::util::errors::{CMError, CustomError};

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

const PASSLEN: usize = 16;

pub fn get_random_pass() -> Result<String, CMError> {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()";
    let chars_arr: Vec<char> = chars.chars().collect();
    let chars_size = chars.len();

    let mut pass = String::new();
    let mut index_buff: [u8; PASSLEN] = [0; PASSLEN];

    let mut rng = match ChaCha20Rng::try_from_os_rng() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return Err(CustomError::new("Error generating Password").into());
        }
    };
    rng.fill(&mut index_buff);

    for i in 0..PASSLEN {
        let mut index: usize = index_buff[i] as usize;
        index = index % chars_size;
        let passchar = &chars_arr[index];

        pass.push(*passchar);
    }

    Ok(pass)
}
