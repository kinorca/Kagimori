mod apis;
mod headers;

use encryption::Encryptor;

pub struct TransitStatelessHttpServer<L> {
    encryptor: Encryptor<L>,
}

impl<L> TransitStatelessHttpServer<L> {
    pub fn new(encryptor: Encryptor<L>) -> Self {
        Self { encryptor }
    }

    pub async fn run(self) {}
}
