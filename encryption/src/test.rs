#[macro_export]
macro_rules! predefined_tests {
    ($create_sut:expr) => {
        #[::tokio::test]
        async fn test_encrypt_decrypt() {
            let encryptor = $create_sut();
            let plaintext = b"test data";

            let ciphertext = $crate::Encryptor::encrypt(&encryptor, plaintext)
                .await
                .expect("Cannot encrypt data");
            let decrypted_plaintext = $crate::Encryptor::decrypt(&encryptor, &ciphertext)
                .await
                .expect("Cannot decrypt data");

            assert_eq!(decrypted_plaintext, plaintext);
        }
    };
}
