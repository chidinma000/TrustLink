//! Shared data types and error definitions for TrustLink.

use soroban_sdk::{contracterror, contracttype, Address, Bytes, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimTypeInfo {
    pub claim_type: String,
    pub description: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuerMetadata {
    pub name: String,
    pub url: String,
    pub description: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub id: String,
    pub issuer: Address,
    pub subject: Address,
    pub claim_type: String,
    pub timestamp: u64,
    pub expiration: Option<u64>,
    pub revoked: bool,
    pub metadata: Option<String>,
    pub valid_from: Option<u64>,
    pub imported: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttestationStatus {
    Valid,
    Expired,
    Revoked,
    Pending,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    NotFound = 4,
    DuplicateAttestation = 5,
    AlreadyRevoked = 6,
    Expired = 7,
    InvalidValidFrom = 8,
    InvalidExpiration = 9,
    MetadataTooLong = 10,
    InvalidTimestamp = 11,
}

impl Attestation {
    pub fn generate_id(
        env: &Env,
        issuer: &Address,
        subject: &Address,
        claim_type: &String,
        timestamp: u64,
    ) -> String {
        let issuer_str = issuer.to_string();
        let subject_str = subject.to_string();

        let issuer_len = issuer_str.len() as usize;
        let subject_len = subject_str.len() as usize;
        let claim_len = claim_type.len() as usize;

        let mut issuer_buf = [0u8; 64];
        let mut subject_buf = [0u8; 64];
        let mut claim_buf = [0u8; 256];

        issuer_str.copy_into_slice(&mut issuer_buf[..issuer_len]);
        subject_str.copy_into_slice(&mut subject_buf[..subject_len]);
        claim_type.copy_into_slice(&mut claim_buf[..claim_len]);

        let mut payload = Bytes::new(env);
        payload.append(&Bytes::from_slice(env, &issuer_buf[..issuer_len]));
        payload.append(&Bytes::from_slice(env, &subject_buf[..subject_len]));
        payload.append(&Bytes::from_slice(env, &claim_buf[..claim_len]));
        payload.append(&Bytes::from_slice(env, &timestamp.to_be_bytes()));

        let hash = env.crypto().sha256(&payload).to_array();
        const HEX: &[u8; 16] = b"0123456789abcdef";

        let mut hex = [0u8; 32];
        for i in 0..16 {
            hex[i * 2] = HEX[(hash[i] >> 4) as usize];
            hex[i * 2 + 1] = HEX[(hash[i] & 0x0f) as usize];
        }

        String::from_bytes(env, &hex)
    }

    pub fn get_status(&self, current_time: u64) -> AttestationStatus {
        if let Some(valid_from) = self.valid_from {
            if current_time < valid_from {
                return AttestationStatus::Pending;
            }
        }

        if self.revoked {
            return AttestationStatus::Revoked;
        }

        if let Some(expiration) = self.expiration {
            if current_time >= expiration {
                return AttestationStatus::Expired;
            }
        }

        AttestationStatus::Valid
    }
}
