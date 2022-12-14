// Set of libraries for privacy-preserving networking apps
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Rajarshi Maitra
//     Dr. Maxim Orlovsky <orlovsky@cyphernet.org>
//
// Copyright 2022-2023 Cyphernet Association, Switzerland
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};

use super::EncryptionError;

pub const TAG_SIZE: usize = 16;

fn _nonce(nonce: u64) -> Nonce {
    let mut chacha_nonce = [0u8; 12];
    chacha_nonce[4..].copy_from_slice(&nonce.to_le_bytes());
    *Nonce::from_slice(&chacha_nonce[..])
}

fn _cypher(key: &[u8]) -> ChaCha20Poly1305 {
    let key = Key::from_slice(key);
    ChaCha20Poly1305::new(key)
}

/// Encrypt a plaintext with associated data using the key and nonce.
///
/// # Returns
///
/// Returns the encrypted msg, which is also copied to ciphertext array, if
/// provided.
///
/// # Panics
///
/// Function panics if `plaintext` and `cyphertext` have different length.
pub fn encrypt(
    key: &[u8],
    nonce: u64,
    associated_data: &[u8],
    plaintext: &[u8],
    ciphertext: Option<&mut [u8]>,
) -> Result<Vec<u8>, EncryptionError> {
    let payload = Payload {
        msg: plaintext,
        aad: associated_data,
    };
    let encrypted = _cypher(key).encrypt(&_nonce(nonce), payload)?;
    if let Some(e) = ciphertext {
        e.copy_from_slice(&encrypted)
    }
    Ok(encrypted)
}

/// Decrypts the ciphertext with key, nonce and associated data.
///
/// # Returns
///
/// Returns the decrypted msg, which is also copied to plaintext array, if
/// provided.
///
/// # Panics
///
/// Function panics if `plaintext` and `cyphertext` have different length.
pub fn decrypt(
    key: &[u8],
    nonce: u64,
    associated_data: &[u8],
    ciphertext: &[u8],
    plaintext: Option<&mut [u8]>,
) -> Result<Vec<u8>, EncryptionError> {
    let payload = Payload {
        msg: ciphertext,
        aad: associated_data,
    };
    let decrypted = _cypher(key).decrypt(&_nonce(nonce), payload)?;
    if let Some(d) = plaintext {
        d.copy_from_slice(&decrypted)
    }
    Ok(decrypted)
}

#[cfg(test)]
mod test {
    use chacha20poly1305::aead::{Aead, AeadInPlace};
    use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};

    #[test]
    fn test1() {
        // Encrypt decrypt a plain text
        let key = Key::from_slice(b"an example very very secret key."); // 32-bytes
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message

        let ciphertext =
            cipher.encrypt(nonce, b"plaintext message".as_ref()).expect("encryption failure!"); // NOTE: handle this error to avoid panics!
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, b"plaintext message");
    }

    #[test]
    fn test2() {
        let key = Key::from_slice(b"an example very very secret key.");
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = Nonce::from_slice(b"unique nonce"); // 128-bits; unique per message

        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(b"plaintext message");

        // Encrypt `buffer` in-place, replacing the plaintext contents with
        // ciphertext
        cipher.encrypt_in_place(nonce, b"", &mut buffer).expect("encryption failure!");

        // `buffer` now contains the message ciphertext
        assert_ne!(&buffer, b"plaintext message");

        // Decrypt `buffer` in-place, replacing its ciphertext context with the
        // original plaintext
        cipher.decrypt_in_place(nonce, b"", &mut buffer).expect("decryption failure!");
        assert_eq!(&buffer, b"plaintext message");
    }
}
