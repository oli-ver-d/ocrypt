pub mod encryption;

use std::fs::File;
use std::io::{Read, Write};
use anyhow::Result;
use anyhow::anyhow;
use encryption::encrypt;

pub fn encrypt_file(file_name: &str, key: &[u8; 32]) -> Result<()> {
    let mut input_file: File = File::open(file_name)?;
    let mut output_file: File = File::create(format!("{file_name}.ocrypt"))?;

    let mut buffer = [0u8; 16];

    while let Ok(bytes_read) = input_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if bytes_read == 16 {
            encrypt::encrypt_block(key, &mut buffer);
            output_file.write_all(&buffer)?;
        } else {
            // PAD this shit
            let bytes_to_pad = 16 - bytes_read;
            for byte in buffer.iter_mut().skip(bytes_read) {
                *byte = bytes_to_pad as u8;
            }
            encrypt::encrypt_block(key, &mut buffer);
            output_file.write_all(&buffer)?;
        }

    }

    Ok(())
}

pub fn string_to_fixed_array(input: &str) -> Result<[u8; 32]> {
    let bytes = input.as_bytes();
    if bytes.len() < 32 {
        return Err(anyhow!("String is not long enough."));
    }

    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes[..32]);
    Ok(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_file() {
        let key: [u8; 32] = [
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 
            0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
            0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 
            0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
        ];

        let file_name = "test.txt";

        let _ = encrypt_file(file_name, &key);
    }
}
