use clap::{Arg, ArgAction, Command};
use std::{io, path::Path};
use ocrypt::*;

fn main() {
    // it'll be so it be like it be so yeah it be like yeah itll be yeah so basically yeah itll be
    // yeah or smth like yeah
    // anyway
    //
    // ocrypt --encrypt file.txt 
    // then prompt for key
    // OR
    // ocrypt --decrypt file.txt --key KEY
    // so set up clap accordingly 

    let encrypt_arg = Arg::new("encrypt")
        .action(ArgAction::SetTrue)
        .long("encrypt")
        .short('e')
        .conflicts_with("decrypt")
        .required_unless_present("decrypt")
        .help("Encrypts a file with a given key (either specified or prompted for)");

    let decrypt_arg = Arg::new("decrypt")
        .action(ArgAction::SetTrue)
        .long("decrypt")
        .short('d')
        .conflicts_with("encrypt")
        .required_unless_present("encrypt")
        .help("Decrypts a file with a given key (either specified or prompted for)");

    let key_arg = Arg::new("key")
        .long("key")
        .short('k')
        .value_name("KEY")
        .help("The encryption key.");

    let filename_arg = Arg::new("filename")
        .required(true);

    let ocrypt = Command::new("Ocrypt")
        .version("0.1")
        .author("swag")
        .about("Encryption tool")
        .arg(encrypt_arg)
        .arg(decrypt_arg)
        .arg(key_arg)
        .arg(filename_arg)
        .get_matches();

    // validate if file exists
    let file_name = ocrypt.get_one::<String>("filename").unwrap();
    if !Path::new(file_name).exists() {
        panic!("Specified file does not exist in this program's opinion.");
    }

    // if key specified, set, otherwise prompt
    let key: String = if let Some(in_key) = ocrypt.get_one::<String>("key") {
        in_key.to_string()
    } else {
        println!("Enter the key to use: ");
        let mut in_key = String::new();
        io::stdin()
            .read_line(&mut in_key)
            .expect("Failed to read line");
        in_key.trim().to_string()
    };

    let encrypt = ocrypt.get_one::<bool>("encrypt").copied().unwrap_or(false);

    if encrypt {
        println!("Encrypting {file_name} with key {key}");
        let key: [u8; 32] = string_to_fixed_array(&key).expect("Key is not long enough, must be at least 256 bits");         
        match encrypt_file(file_name, &key) {
            Ok(_) => println!("Successfully written to {file_name}.ocrypt"),
            Err(e) => println!("Failed to encrypt, error: {e}")
        };
    } else {
        println!("Decrypting {file_name} with key {key}");
    }
}
