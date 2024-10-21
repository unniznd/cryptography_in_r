mod classical_cipher{
    pub mod caesar_cipher;
    pub mod playfair;
}

use std::io;

use crate::classical_cipher::caesar_cipher::caesar_cipher;
use crate::classical_cipher::playfair::play_fair;


fn handle_input(){
    loop{
        println!("\n1. Caesar cipher");
        println!("2. Playfair cipher");
        println!("3. Exit\n");

        let mut option_str: String = String::new();

        println!("Enter which cipher");
        io::stdin()
        .read_line(&mut option_str)
        .expect("Failed to read input");

        if option_str.trim() == "1"{
            let mut input: String = String::new();

            println!("Enter input:");
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

            let _ = input.pop();

            let mut key_str: String = String::new();

            println!("Enter key:");
            io::stdin()
            .read_line(&mut key_str)
            .expect("Failed to read input");

            let key:u8 = key_str.trim().parse().expect("Failed to parse");

            loop{
                println!("\n1. Encrypt");
                println!("2. Decrypt\n");

                let mut is_encrypt: String = String::new();

                println!("Enter option:");
                io::stdin()
                .read_line(&mut is_encrypt)
                .expect("Failed to read input");

                if is_encrypt.trim() == "1"{
                    let result: String = caesar_cipher(input, key, true);
                    println!("Encrypted Text: {}", result);
                    break;
                }else if is_encrypt.trim() == "2"{
                    let result: String = caesar_cipher(input, key, false);
                    println!("Decrypted Text {}", result);
                    break;
                }else{
                    println!("Error: Invalid input")
                }
            }

        }else if option_str.trim() == "2"{
            let mut input: String = String::new();

            println!("Enter input:");
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

            let _ = input.pop();

            let mut key_str: String = String::new();

            println!("Enter key:");
            io::stdin()
            .read_line(&mut key_str)
            .expect("Failed to read input");

            let _ = key_str.pop();

            

            loop{
                println!("\n1. Encrypt");
                println!("2. Decrypt\n");

                let mut is_encrypt: String = String::new();

                println!("Enter option:");
                io::stdin()
                .read_line(&mut is_encrypt)
                .expect("Failed to read input");

                if is_encrypt.trim() == "1"{
                    let result: String = play_fair(input, key_str, true);
                    println!("Encrypted Text: {}", result);
                    break;
                }else if is_encrypt.trim() == "2"{
                    let result: String = play_fair(input, key_str, false);
                    println!("Decrypted Text {}", result);
                    break;
                }else{
                    println!("Error: Invalid input")
                }
            }
            
        }else if option_str.trim() == "3"{
            println!("Bye");
            break;
        }
        else{
            println!("Error: Invalid input\n");
            continue;
        }
    }
}

fn main() {
    handle_input();
}
