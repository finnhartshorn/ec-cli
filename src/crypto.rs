use aes::{Aes128, Aes192, Aes256};
use cbc::{Decryptor, cipher::{BlockDecryptMut, KeyIvInit}};
use crate::error::{EcError, Result};

type Aes128CbcDec = Decryptor<Aes128>;
type Aes192CbcDec = Decryptor<Aes192>;
type Aes256CbcDec = Decryptor<Aes256>;

/// Decrypt AES-CBC encrypted content with PKCS7 padding
///
/// The encryption scheme used by Everybody Codes:
/// - Algorithm: AES-128/192/256-CBC (determined by key length)
/// - IV: First 16 bytes of the key
/// - Padding: PKCS7
/// - Input format: Hex-encoded ciphertext
pub fn decrypt_aes_cbc(ciphertext_hex: &str, key: &str) -> Result<String> {
    use log::debug;

    debug!("Decrypting with key length: {}, ciphertext length: {}", key.len(), ciphertext_hex.len());

    // Decode hex ciphertext
    let ciphertext = hex::decode(ciphertext_hex)?;
    debug!("Decoded ciphertext length: {} bytes", ciphertext.len());

    // Get key bytes
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    // IV is always first 16 bytes
    if key_len < 16 {
        return Err(EcError::DecryptionError(
            format!("Key too short: {key_len} bytes (need at least 16)")
        ));
    }
    let iv: [u8; 16] = key_bytes[..16].try_into()
        .map_err(|e| EcError::DecryptionError(format!("IV conversion failed: {e}")))?;

    debug!("Using AES-{} based on key length", key_len * 8);

    // Decrypt based on key size
    let mut buffer = ciphertext.clone();
    let decrypted = match key_len {
        16 => {
            // AES-128
            let key_array: [u8; 16] = key_bytes.try_into()
                .map_err(|_| EcError::DecryptionError("Key conversion failed".to_string()))?;
            let cipher = Aes128CbcDec::new(&key_array.into(), &iv.into());
            cipher.decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buffer)
                .map_err(|e| EcError::DecryptionError(format!("AES-128 decryption failed: {e}")))?
        },
        24 => {
            // AES-192
            let key_array: [u8; 24] = key_bytes.try_into()
                .map_err(|_| EcError::DecryptionError("Key conversion failed".to_string()))?;
            let cipher = Aes192CbcDec::new(&key_array.into(), &iv.into());
            cipher.decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buffer)
                .map_err(|e| EcError::DecryptionError(format!("AES-192 decryption failed: {e}")))?
        },
        32 => {
            // AES-256
            let key_array: [u8; 32] = key_bytes.try_into()
                .map_err(|_| EcError::DecryptionError("Key conversion failed".to_string()))?;
            let cipher = Aes256CbcDec::new(&key_array.into(), &iv.into());
            cipher.decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buffer)
                .map_err(|e| EcError::DecryptionError(format!("AES-256 decryption failed: {e}")))?
        },
        _ => {
            return Err(EcError::DecryptionError(
                format!("Invalid key length: {key_len} (must be 16, 24, or 32 bytes)")
            ));
        }
    };

    // Convert to string
    String::from_utf8(decrypted.to_vec())
        .map_err(|e| EcError::DecryptionError(format!("UTF-8 conversion failed: {e}")))
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
