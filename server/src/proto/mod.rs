pub(crate) mod kinorca {
    pub mod kagimori {
        pub mod v1 {
            tonic::include_proto!("kinorca.kagimori.v1");
        }
    }
}

pub(crate) mod kubernetes {
    pub mod kms {
        pub mod v2 {
            tonic::include_proto!("v2");
        }
    }
}
