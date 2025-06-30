use sha2::Digest;

pub(crate) fn compute_identifier(algorithm_id: u16, key: &[u8]) -> [u8; 16] {
    let digest = sha2::Sha256::digest(key);
    let aid: [u8; 2] = algorithm_id.to_be_bytes();
    let kid = &digest[0..(16 - 2)];

    let mut identifier = [0u8; 16];
    identifier.copy_from_slice(&aid);
    identifier[2..].copy_from_slice(kid);

    identifier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_identifier_same_algorithm_and_key() {
        let algorithm_id = 0x1234;
        let key = [0u8; 32];
        let identifier1 = compute_identifier(algorithm_id, &key);

        let identifier2 = compute_identifier(algorithm_id, &key);

        assert_eq!(identifier1, identifier2);
    }

    #[test]
    fn test_compute_identifier_difference_key() {
        let algorithm_id = 0x1234;
        let key1 = [0u8; 32];
        let identifier1 = compute_identifier(algorithm_id, &key1);

        let algorithm_id = 0x1234;
        let key2 = [1u8; 32];
        let identifier2 = compute_identifier(algorithm_id, &key2);

        assert_ne!(identifier1, identifier2);
    }

    #[test]
    fn test_compute_identifier_difference_algorithm() {
        let algorithm_id = 0x1234;
        let key = [0u8; 32];
        let identifier1 = compute_identifier(algorithm_id, &key);

        let algorithm_id2 = 0x5678;
        let identifier2 = compute_identifier(algorithm_id2, &key);

        assert_ne!(identifier1, identifier2);
    }
}
