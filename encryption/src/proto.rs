pub(crate) mod kagimori {
    pub mod encryption {
        pub mod key {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/kagimori.encryption.key.v1.rs"));
            }
        }
    }
}
