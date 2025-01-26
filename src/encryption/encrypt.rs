
const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const R_CON: [u8; 10] = [
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];

fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = AES_SBOX[*byte as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let temp = *state;

    // first row no changes

    // the second row
    state[4] = temp[5];
    state[5] = temp[6];
    state[6] = temp[7];
    state[7] = temp[4];

    // the third row
    state[8] = temp[10];
    state[9] = temp[11];
    state[10] = temp[8];
    state[11] = temp[9];

    // the forth row
    state[12] = temp[15];
    state[13] = temp[12];
    state[14] = temp[13];
    state[15] = temp[14];
}

// help
fn gal_mul (a: u8, b: u8) -> u8 {
    let mut result: u8 = 0; // Result of the multiplication
    let mut a = a; // Copy of the first operand
    let mut b = b; // Copy of the second operand

    // Irreducible polynomial for GF(2^8)
    const IRREDUCIBLE_POLY: u8 = 0x1b; // (x^8) + x^4 + x^3 + x + 1

    // Process each bit of the second operand
    while b != 0 {
        // If the least significant bit of b is 1, add the current a to the result
        if (b & 1) != 0 {
            result ^= a; // XOR is used instead of addition in GF(2^8)
        }

        // Shift a to the left, which corresponds to multiplying by x in GF(2^8)
        let high_bit_set = (a & 0x80) != 0; // Check if the high bit (x^7) is set
        a <<= 1; // Multiply a by x

        // If the high bit was set before shifting, reduce a modulo the irreducible polynomial
        if high_bit_set {
            a ^= IRREDUCIBLE_POLY; // Perform the reduction
        }

        // Shift b to the right, moving to the next bit
        b >>= 1;
    }

    result
}

fn mix_columns(state: &mut [u8; 16]) {
    let temp = *state;
    
    // column 1
    state[0] = gal_mul(temp[0], 0x02) ^ gal_mul(temp[4], 0x03) ^ temp[8] ^ temp[12];
    state[4] = temp[0] ^ gal_mul(temp[4], 0x02) ^ gal_mul(temp[8], 0x03) ^ temp[12];
    state[8] = temp[0] ^ temp[4] ^ gal_mul(temp[8], 0x02) ^ gal_mul(temp[12], 0x03);
    state[12] = gal_mul(temp[0], 0x03) ^ temp[4] ^ temp[8] ^ gal_mul(temp[12], 0x02);

    // column 2
    state[1] = gal_mul(temp[1], 0x02) ^ gal_mul(temp[5], 0x03) ^ temp[9] ^ temp[13];
    state[5] = temp[1] ^ gal_mul(temp[5], 0x02) ^ gal_mul(temp[9], 0x03) ^ temp[13];
    state[9] = temp[1] ^ temp[5] ^ gal_mul(temp[9], 0x02) ^ gal_mul(temp[13], 0x03);
    state[13] = gal_mul(temp[1], 0x03) ^ temp[5] ^ temp[9] ^ gal_mul(temp[13], 0x02);
    
    // column 3
    state[2] = gal_mul(temp[2], 0x02) ^ gal_mul(temp[5], 0x03) ^ temp[10] ^ temp[14];
    state[6] = temp[2] ^ gal_mul(temp[5], 0x02) ^ gal_mul(temp[10], 0x03) ^ temp[14];
    state[10] = temp[2] ^ temp[5] ^ gal_mul(temp[10], 0x02) ^ gal_mul(temp[14], 0x03);
    state[14] = gal_mul(temp[2], 0x03) ^ temp[5] ^ temp[10] ^ gal_mul(temp[14], 0x02);

    // column 4 - todo!
    state[3] = gal_mul(temp[3], 0x02) ^ gal_mul(temp[7], 0x03) ^ temp[11] ^ temp[15];
    state[7] = temp[3] ^ gal_mul(temp[7], 0x02) ^ gal_mul(temp[11], 0x03) ^ temp[15];
    state[11] = temp[3] ^ temp[7] ^ gal_mul(temp[11], 0x02) ^ gal_mul(temp[15], 0x03);
    state[15] = gal_mul(temp[3], 0x03) ^ temp[7] ^ temp[11] ^ gal_mul(temp[15], 0x02);
}

fn key_expansion(key: &[u8; 32], expanded_key: &mut [u8; 240]) {
    // 4 words (4 bytes) for each of the Nr + 1 applications of addroundkeys()
    // for AES 256 (this one) it is (Nr + 1) * 4 = (14 + 1) * 4 = 60 words = 240 bytes = 1920 bits
    // Nk = 8, the number of words in the key (256 bits)
    // Nr = 14, the number of rounds
    // key length is 32 bytes, 8 words
    // the first 8 words of the expanded key is just the original key
    
    expanded_key[..32].copy_from_slice(key);

    let mut temp = [0u8; 4];

    for i in 8..60 {
        temp.copy_from_slice(&expanded_key[(i - 1) * 4..i * 4]);

        if i % 8 == 0 {
            rot_word(&mut temp);
            sub_word(&mut temp);
            temp[0] ^= R_CON[i / 8 - 1];
        } else if i % 8 == 4 {
            sub_word(&mut temp);
        }

        for j in 0..4 {
            expanded_key[i * 4 + j] = temp[j] ^ expanded_key[(i - 8) * 4 + j];
        }
    }
}

fn rot_word(word: &mut [u8; 4]) {
    let temp = word[0];
    word[0] = word[1];
    word[1] = word[2];
    word[2] = word[3];
    word[3] = temp;
}

fn sub_word(word: &mut [u8; 4]) {
    for byte in word.iter_mut() {
        *byte = AES_SBOX[*byte as usize];
    }
}

fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
    for i in 0..16 {
        state[i] ^= round_key[i];
    }
}

fn encrypt_block(key: &[u8; 32], block: &mut [u8; 16]) {
    let mut expanded_key = [0u8; 240];
    key_expansion(key, &mut expanded_key);

    add_round_key(block, &expanded_key[0..16]);
    for i in 1..14 {
        sub_bytes(block);
        shift_rows(block);
        mix_columns(block);
        add_round_key(block, &expanded_key[i * 16..(i + 1) * 16]);
    }
    sub_bytes(block);
    shift_rows(block);
    add_round_key(block, &expanded_key[224..240]);
}

fn pad(data: &mut Vec<u8>, block_size: usize) {
    let padding_len = block_size - (data.len() % block_size);
    data.extend(vec![padding_len as u8; padding_len]);
}

pub fn encrypt(key: &[u8; 32], data: &mut [u8]) {
    let block_size = 16;
    let mut temp_block = [0u8; 16];

    for block in data.chunks_exact_mut(16) {
        encrypt_block(key, block.try_into().expect("Somehow array is not 16 long wtf"));
    }
    
    todo!();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_bytes() {
        let mut test_values: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        sub_bytes(&mut test_values);
        let expected_values: [u8; 16] = [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76];
        assert_eq!(test_values, expected_values);
    }

    #[test]
    fn test_shift_rows() {
        let mut test_values: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        shift_rows(&mut test_values);
        let expected_values: [u8; 16] = [0, 1, 2, 3, 5, 6, 7, 4, 10, 11, 8, 9, 15, 12, 13, 14];
        assert_eq!(test_values, expected_values);
    }

    #[test]
    fn test_rot_word() {
        let mut test_values: [u8; 4] = [0x63, 0x7b, 0xe2, 0x76];
        rot_word(&mut test_values);
        let expected_values: [u8; 4] = [0x7b, 0xe2, 0x76, 0x63];
        assert_eq!(test_values, expected_values);
    }

    #[test]
    fn test_sub_word() {
        let mut test_values: [u8; 4] = [0, 1, 2, 3];
        sub_word(&mut test_values);
        let expected_values: [u8; 4] = [0x63, 0x7c, 0x77, 0x7b];
        assert_eq!(test_values, expected_values);
    }

    #[test]
    fn test_key_expansion() {
        let test_key: [u8; 32] = [
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 
            0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
            0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 
            0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
        ];

        let mut expanded_key = [0u8; 240];
        key_expansion(&test_key, &mut expanded_key);
        for chunk in expanded_key.chunks(4) {
            for byte in chunk {
                print!("{:02x}", byte);
            }
            println!(); 
        }
        
    }

    #[test]
    fn test_add_round_key() {
        let mut test_values: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let round_key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        add_round_key(&mut test_values, &round_key);
        let expected_values: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];        
        assert_eq!(test_values, expected_values);
    }

    #[test]
    fn test_encrypt_block() {
        let key: [u8; 32] = [
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 
            0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
            0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 
            0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
        ];

        let mut block: [u8; 16] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ];

        encrypt_block(&key, &mut block);
    }
}
