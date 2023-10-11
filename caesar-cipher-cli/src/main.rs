/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/

use caeser_cipher_cli::{decrypt, encrypt, int_to_ascii};
use clap::Parser;

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

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}

fn main() {
    let args = Args::parse();

    if args.encrypt {
        let encrypted = encrypt(&args.message, args.shift);
        println!("Encrypted message: {}", encrypted);
    } else if args.decrypt {
        let decrypted = decrypt(&args.message, args.shift);
        println!("Decrypted message: {}", decrypted);
    } else if args.int_to_ascii {
        if let Ok(integer_value) = args.message.parse::<u8>() {
            match int_to_ascii(integer_value) {
                Ok(ascii_char) => {
                    println!(
                        "Integer: {} corresponds to ASCII character: {}",
                        integer_value, ascii_char
                    );
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        } else {
            eprintln!("Error: Invalid integer value provided.");
        }
    } else {
        eprintln!("Please specify either --encrypt, --decrypt, or --int_to_ascii");
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
