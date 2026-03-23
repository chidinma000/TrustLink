use soroban_sdk::{contracterror, contracttype, Address, Env, String};

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
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttestationStatus {
    Valid,
    Expired,
    Revoked,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    NotFound = 4,
    DuplicateAttestation = 5,
    AlreadyRevoked = 6,
    Expired = 7,
}

impl Attestation {
    pub fn generate_id(
        env: &Env,
        issuer: &Address,
        subject: &Address,
        claim_type: &String,
        timestamp: u64,
    ) -> String {
        use soroban_sdk::xdr::ToXdr;
        let data = (issuer.clone(), subject.clone(), claim_type.clone(), timestamp);
        let xdr_bytes = data.to_xdr(env);
        let hash = env.crypto().sha256(&xdr_bytes);
        let hash_bytes = hash.to_array();
        // Use first 16 bytes as ID
        String::from_bytes(env, &hash_bytes[..16])
    }

    pub fn get_status(&self, current_time: u64) -> AttestationStatus {
        if self.revoked {
            return AttestationStatus::Revoked;
        }
        if let Some(exp) = self.expiration {
            if current_time > exp {
                return AttestationStatus::Expired;
            }
        }
        AttestationStatus::Valid
    }
}
