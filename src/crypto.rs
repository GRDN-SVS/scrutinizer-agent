use sodiumoxide::crypto;

/// Struct used to manage all the encryption logic
#[derive(Clone)]
pub struct Decrypter {
    pub public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl Decrypter {
    /// Creates a new `Encrypter` instance with private and public keys
    pub fn new() -> Decrypter {
        let (public_key, private_key) = crypto::box_::gen_keypair();

        Decrypter {
            public_key: public_key.0.to_vec(),
            private_key: private_key.0.to_vec(),
        }
    }

    /// Verifies that the message has in fact passed through a certain
    /// agent
    pub fn verify(&self, msg: &[u8], other_pk: &[u8]) -> Vec<u8> {
        let public_verify_key =
            crypto::sign::PublicKey::from_slice(other_pk).expect("Could not create the verify key");

        crypto::sign::verify(msg, &public_verify_key).expect("Could not verify the message")
    }

    /// Decrypts the message using another agent's public key,
    /// in order to gain access to the original value of the shared secret
    pub fn open(&self, msg: &[u8], nonce: &[u8], other_pk: &[u8]) -> Vec<u8> {
        let nonce = crypto::box_::Nonce::from_slice(nonce).unwrap();
        let other_pk = crypto::box_::PublicKey::from_slice(other_pk).unwrap();
        let my_secret_key = crypto::box_::SecretKey::from_slice(&self.private_key).unwrap();

        crypto::box_::open(msg, &nonce, &other_pk, &my_secret_key)
            .expect("Could not open the message")
    }
}
