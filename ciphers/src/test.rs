// Copyright 2025 SiLeader.
//
// This file is part of Kagimori.
//
// Kagimori is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any later version.
//
// Kagimori is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Kagimori.
// If not, see <https://www.gnu.org/licenses/>.

#[macro_export]
macro_rules! predefined_tests {
    ($create_sut:expr) => {
        #[::tokio::test]
        async fn test_encrypt_decrypt() {
            let encryptor = $create_sut();
            let plaintext = b"test data";

            let ciphertext = $crate::Cipher::encrypt(&encryptor, plaintext)
                .await
                .expect("Cannot encrypt data");
            let decrypted_plaintext = $crate::Cipher::decrypt(&encryptor, &ciphertext)
                .await
                .expect("Cannot decrypt data");

            assert_eq!(decrypted_plaintext, plaintext);
        }
    };
}
