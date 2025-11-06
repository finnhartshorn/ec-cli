use aes::Aes128;
use cbc::{Decryptor, cipher::{BlockDecryptMut, KeyIvInit}};
use crate::error::{EcError, Result};

type Aes128CbcDec = Decryptor<Aes128>;

/// Decrypt AES-CBC encrypted content with PKCS7 padding
///
/// The encryption scheme used by Everybody Codes:
/// - Algorithm: AES-128-CBC
/// - IV: First 16 bytes of the key
/// - Padding: PKCS7
/// - Input format: Hex-encoded ciphertext
pub fn decrypt_aes_cbc(ciphertext_hex: &str, key: &str) -> Result<String> {
    // Decode hex ciphertext
    let ciphertext = hex::decode(ciphertext_hex)?;

    // Prepare key (first 16 bytes of the provided key)
    let key_bytes = key.as_bytes();
    if key_bytes.len() < 16 {
        return Err(EcError::DecryptionError(
            format!("Key too short: {} bytes (need at least 16)", key_bytes.len())
        ));
    }
    let key_array: [u8; 16] = key_bytes[..16].try_into()
        .map_err(|e| EcError::DecryptionError(format!("Key conversion failed: {}", e)))?;

    // IV is the same as key (first 16 bytes)
    let iv: [u8; 16] = key_array;

    // Create decryptor
    let cipher = Aes128CbcDec::new(&key_array.into(), &iv.into());

    // Decrypt with PKCS7 unpadding
    let mut buffer = ciphertext.clone();
    let decrypted = cipher
        .decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buffer)
        .map_err(|e| EcError::DecryptionError(format!("Decryption failed: {}", e)))?;

    // Convert to string
    String::from_utf8(decrypted.to_vec())
        .map_err(|e| EcError::DecryptionError(format!("UTF-8 conversion failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        // This is a simplified test - in real usage, you'd need actual encrypted data
        // from the Everybody Codes API
        let key = "0123456789abcdef0123456789abcdef";
        let result = decrypt_aes_cbc("invalid_hex", key);
        assert!(result.is_err());
    }
}
