use clap::{Arg, ArgAction, Command};

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


}
