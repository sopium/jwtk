//! Enum of HMAC / EC / RSA / Ed Keys.

use openssl::pkey::{Id, PKey};

use crate::{
    ecdsa::{EcdsaPrivateKey, EcdsaPublicKey},
    eddsa::{Ed25519PrivateKey, Ed25519PublicKey},
    jwk::Jwk,
    rsa::{RsaAlgorithm, RsaPrivateKey, RsaPublicKey},
    Error, PublicKeyToJwk, Result, SigningKey, VerificationKey,
};

#[non_exhaustive]
#[derive(Debug)]
pub enum SomePrivateKey {
    Ed25519(Ed25519PrivateKey),
    Ecdsa(EcdsaPrivateKey),
    Rsa(RsaPrivateKey),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum SomePublicKey {
    Ed25519(Ed25519PublicKey),
    Ecdsa(EcdsaPublicKey),
    Rsa(RsaPublicKey),
}

impl SomePrivateKey {
    /// Read an RSA/EC/Ed25519 private key from PEM.
    ///
    /// For an EC/Ed25519 private key, algorithm is deduced from the curve, e.g.
    /// P-256 -> ES256.
    ///
    /// For an RSA private key, `if_rsa_algorithm` is used.
    pub fn from_pem(pem: &[u8], if_rsa_algorithm: RsaAlgorithm) -> Result<Self> {
        let pk = PKey::private_key_from_pem(pem)?;

        match pk.id() {
            Id::RSA => {
                let k = RsaPrivateKey::from_pkey(pk, if_rsa_algorithm)?;
                Ok(Self::Rsa(k))
            }
            Id::EC => {
                let k = EcdsaPrivateKey::from_pkey(pk)?;
                Ok(Self::Ecdsa(k))
            }
            Id::ED25519 => {
                let k = Ed25519PrivateKey::from_pkey(pk)?;
                Ok(Self::Ed25519(k))
            }
            _ => Err(Error::UnsupportedOrInvalidKey),
        }
    }

    pub fn private_key_to_pem_pkcs8(&self) -> Result<Vec<u8>> {
        match self {
            SomePrivateKey::Ed25519(ed) => ed.private_key_to_pem_pkcs8(),
            SomePrivateKey::Ecdsa(ec) => ec.private_key_to_pem_pkcs8(),
            SomePrivateKey::Rsa(rsa) => rsa.private_key_to_pem_pkcs8(),
        }
    }

    pub fn public_key_to_pem(&self) -> Result<Vec<u8>> {
        match self {
            SomePrivateKey::Ed25519(ed) => ed.public_key_to_pem(),
            SomePrivateKey::Ecdsa(ec) => ec.public_key_to_pem(),
            SomePrivateKey::Rsa(rsa) => rsa.public_key_to_pem(),
        }
    }
}

impl PublicKeyToJwk for SomePrivateKey {
    fn public_key_to_jwk(&self) -> Result<Jwk> {
        match self {
            SomePrivateKey::Ed25519(ed) => ed.public_key_to_jwk(),
            SomePrivateKey::Ecdsa(ec) => ec.public_key_to_jwk(),
            SomePrivateKey::Rsa(rsa) => rsa.public_key_to_jwk(),
        }
    }
}

impl SomePublicKey {
    /// Read an RSA/EC/Ed25519 public key from PEM.
    ///
    /// For an EC/Ed25519 public key, algorithm is deduced from the curve, e.g.
    /// P-256 -> ES256.
    ///
    /// For an RSA public key, signatures generated by any RSA algorithms can be
    /// verified.
    pub fn from_pem(pem: &[u8]) -> Result<Self> {
        let pk = PKey::public_key_from_pem(pem)?;
        match pk.id() {
            Id::RSA => {
                let k = RsaPublicKey::from_pkey(pk, None)?;
                Ok(Self::Rsa(k))
            }
            Id::EC => {
                let k = EcdsaPublicKey::from_pkey(pk)?;
                Ok(Self::Ecdsa(k))
            }
            Id::ED25519 => {
                let k = Ed25519PublicKey::from_pkey(pk)?;
                Ok(Self::Ed25519(k))
            }
            _ => Err(Error::UnsupportedOrInvalidKey),
        }
    }

    pub fn to_pem(&self) -> Result<Vec<u8>> {
        match self {
            SomePublicKey::Ed25519(ed) => ed.to_pem(),
            SomePublicKey::Ecdsa(ec) => ec.to_pem(),
            SomePublicKey::Rsa(rsa) => rsa.to_pem(),
        }
    }
}

impl SigningKey for SomePrivateKey {
    fn alg(&self) -> &'static str {
        match self {
            SomePrivateKey::Ed25519(ed) => ed.alg(),
            SomePrivateKey::Ecdsa(ec) => ec.alg(),
            SomePrivateKey::Rsa(rsa) => rsa.alg(),
        }
    }

    fn sign(&self, v: &[u8]) -> crate::Result<smallvec::SmallVec<[u8; 64]>> {
        match self {
            SomePrivateKey::Ed25519(ed) => ed.sign(v),
            SomePrivateKey::Ecdsa(ec) => ec.sign(v),
            SomePrivateKey::Rsa(rsa) => rsa.sign(v),
        }
    }
}

impl VerificationKey for SomePrivateKey {
    fn verify(&self, v: &[u8], sig: &[u8], alg: &str) -> crate::Result<()> {
        match self {
            SomePrivateKey::Ed25519(ed) => ed.verify(v, sig, alg),
            SomePrivateKey::Ecdsa(ec) => ec.verify(v, sig, alg),
            SomePrivateKey::Rsa(rsa) => rsa.verify(v, sig, alg),
        }
    }
}

impl VerificationKey for SomePublicKey {
    fn verify(&self, v: &[u8], sig: &[u8], alg: &str) -> crate::Result<()> {
        match self {
            SomePublicKey::Ed25519(ed) => ed.verify(v, sig, alg),
            SomePublicKey::Ecdsa(ec) => ec.verify(v, sig, alg),
            SomePublicKey::Rsa(rsa) => rsa.verify(v, sig, alg),
        }
    }
}

impl PublicKeyToJwk for SomePublicKey {
    fn public_key_to_jwk(&self) -> Result<Jwk> {
        match self {
            SomePublicKey::Ed25519(ed) => ed.public_key_to_jwk(),
            SomePublicKey::Ecdsa(ec) => ec.public_key_to_jwk(),
            SomePublicKey::Rsa(rsa) => rsa.public_key_to_jwk(),
        }
    }
}