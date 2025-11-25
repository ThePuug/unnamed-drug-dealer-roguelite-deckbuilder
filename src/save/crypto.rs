// HMAC signature operations for save file integrity.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::OnceLock;
use serde::Deserialize;

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
struct SaveKeyConfig {
    key: String,
}

// Key loaded from assets/secrets/save_key.ron
static SAVE_KEY: OnceLock<Vec<u8>> = OnceLock::new();

const DEFAULT_KEY: &[u8] = b"development-only-key-replace-in-production";

fn get_key() -> &'static [u8] {
    SAVE_KEY.get_or_init(|| {
        match std::fs::read_to_string("assets/secrets/save_key.ron") {
            Ok(contents) => {
                match ron::from_str::<SaveKeyConfig>(&contents) {
                    Ok(config) if config.key.len() >= 32 => config.key.into_bytes(),
                    _ => {
                        #[cfg(debug_assertions)]
                        eprintln!("Warning: Invalid save_key.ron format. Using default key.");
                        DEFAULT_KEY.to_vec()
                    }
                }
            }
            _ => {
                #[cfg(debug_assertions)]
                eprintln!("Warning: assets/secrets/save_key.ron not found. Using default key.");
                DEFAULT_KEY.to_vec()
            }
        }
    })
}

/// Generate HMAC-SHA256 signature for data
pub fn sign(data: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(get_key())
        .expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().into()
}

/// Verify HMAC-SHA256 signature
pub fn verify(data: &[u8], signature: &[u8; 32]) -> bool {
    let mut mac = HmacSha256::new_from_slice(get_key())
        .expect("HMAC can take key of any size");
    mac.update(data);
    mac.verify_slice(signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let data = b"test data for signing";
        let signature = sign(data);
        assert!(verify(data, &signature));
    }

    #[test]
    fn test_tampered_data_fails_verification() {
        let data = b"original data";
        let signature = sign(data);

        let tampered = b"tampered data";
        assert!(!verify(tampered, &signature));
    }

    #[test]
    fn test_tampered_signature_fails_verification() {
        let data = b"test data";
        let mut signature = sign(data);
        signature[0] = signature[0].wrapping_add(1);

        assert!(!verify(data, &signature));
    }

    #[test]
    fn test_different_data_different_signatures() {
        let sig1 = sign(b"data one");
        let sig2 = sign(b"data two");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_same_data_same_signature() {
        let data = b"consistent data";
        let sig1 = sign(data);
        let sig2 = sign(data);
        assert_eq!(sig1, sig2);
    }
}
