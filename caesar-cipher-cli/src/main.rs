/*

To encrypt:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

To transfer a int value to a correspond character in ASCII table, replace "int" with your number:

cargo run -- --int-to-ascii --message int

*/

use caeser_cipher_cli::{decrypt, encrypt, int_to_ascii};
use clap::Parser;
use std::fs;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// int to assii
    #[arg(short, long)]
    int_to_ascii: bool,

    /// The message to encrypt, decrypt or transfer
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}

// fn main() {
//     let args = Args::parse();

//     if args.encrypt {
//         let encrypted = encrypt(&args.message, args.shift);
//         println!("Encrypted message: {}", encrypted);
//     } else if args.decrypt {
//         let decrypted = decrypt(&args.message, args.shift);
//         println!("Decrypted message: {}", decrypted);
//     } else if args.int_to_ascii {
//         if let Ok(integer_value) = args.message.parse::<u8>() {
//             match int_to_ascii(integer_value) {
//                 Ok(ascii_char) => {
//                     println!(
//                         "Integer: {} corresponds to ASCII character: {}",
//                         integer_value, ascii_char
//                     );
//                 }
//                 Err(err) => {
//                     eprintln!("Error: {}", err);
//                 }
//             }
//         } else {
//             eprintln!("Error: Invalid integer value provided.");
//         }
//     } else {
//         eprintln!("Please specify either --encrypt, --decrypt, or --int_to_ascii");
//     }
// }

fn main() {
    let args = Args::parse();

    let input_data = match read_input(&args.message) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading input: {}", err);
            return;
        }
    };

    if args.encrypt {
        let encrypted = encrypt(&input_data, args.shift);
        println!("Encrypted message: {}", encrypted);
    } else if args.decrypt {
        let decrypted = decrypt(&input_data, args.shift);
        println!("Decrypted message: {}", decrypted);
    } else if args.int_to_ascii {
        if let Ok(integer_value) = input_data.parse::<u8>() {
            match int_to_ascii(integer_value) {
                Ok(ascii_char) => {
                    println!("Integer: {} corresponds to ASCII character: {}", integer_value, ascii_char);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        } else {
            eprintln!("Error: Invalid integer value provided.");
        }
    } else {
        eprintln!("Please specify either --encrypt, --decrypt, or --int-to-ascii");
    }
}

fn read_input(input: &str) -> Result<String, std::io::Error> {
    if input.ends_with(".txt") {
        // Read from a ".txt" file
        fs::read_to_string(input)
    } else {
        // Otherwise, treat it as a direct input
        Ok(input.to_string())
    }
}

// run it
// fn main() {
//     let args = Args::parse();
//     if args.encrypt {
//         println!("{}", encrypt(&args.message, args.shift));
//     } else if args.decrypt {
//         println!("{}", decrypt(&args.message, args.shift));
//     } else {
//         println!("Please specify either --encrypt or --decrypt");
//     }

// }
