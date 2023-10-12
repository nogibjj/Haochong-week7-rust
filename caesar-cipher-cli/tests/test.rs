#[cfg(test)]
mod tests {
    use caeser_cipher_cli::{decrypt, encrypt, int_to_ascii};
    
    #[test]
    fn test_encrypt_decrypt() {
        let text = "Harry Xia";
        let shift = 3;
        let encrypted = encrypt(text, shift);
        let decrypted = decrypt(&encrypted, shift);
        
        assert_eq!(text, decrypted);
    }

    #[test]
    fn test_int_to_ascii() {
        let integer_value = 65;
        let expected_ascii = 'A';
        let result = int_to_ascii(integer_value);
        
        assert_eq!(Ok(expected_ascii), result);
    }
    
}
