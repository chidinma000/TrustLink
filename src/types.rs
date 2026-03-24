use soroban_sdk::{contracterror, contracttype, Address, Env, String};

/// A single attestation record stored on-chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    /// Deterministic hash-based identifier for this attestation.
    pub id: String,
    /// Address that created the attestation.
    pub issuer: Address,
    /// Address the attestation is about.
    pub subject: Address,
    /// Free-form claim label, e.g. `"KYC_PASSED"`.
    pub claim_type: String,
    /// Ledger timestamp (seconds) when the attestation was created.
    pub timestamp: u64,
    /// Optional Unix timestamp after which the attestation is expired.
    pub expiration: Option<u64>,
    /// `true` if the issuer has explicitly revoked this attestation.
    pub revoked: bool,
}

/// The current validity state of an attestation.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttestationStatus {
    /// Attestation is active and has not expired.
    Valid,
    /// Attestation has passed its expiration timestamp.
    Expired,
    /// Attestation was explicitly revoked by its issuer.
    Revoked,
}

/// Errors returned by TrustLink contract functions.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// [`initialize`](crate::TrustLinkContract::initialize) was called more than once.
    AlreadyInitialized = 1,
    /// A function was called before [`initialize`](crate::TrustLinkContract::initialize).
    NotInitialized = 2,
    /// The caller lacks the required admin or issuer role.
    Unauthorized = 3,
    /// No attestation exists with the requested ID.
    NotFound = 4,
    /// An attestation with the same deterministic ID already exists.
    DuplicateAttestation = 5,
    /// The attestation has already been revoked.
    AlreadyRevoked = 6,
    /// The attestation has passed its expiration timestamp.
    Expired = 7,
}

impl Attestation {
    /// Generate a deterministic attestation ID by SHA-256 hashing the tuple
    /// `(issuer, subject, claim_type, timestamp)`.
    ///
    /// The first 16 bytes of the hash are used as the ID to keep it compact
    /// while still being collision-resistant for practical purposes.
    ///
    /// # Parameters
    /// - `issuer` — issuer address.
    /// - `subject` — subject address.
    /// - `claim_type` — claim label string.
    /// - `timestamp` — ledger timestamp at creation time.
    ///
    /// # Returns
    /// A [`String`] containing the raw 16-byte ID.
    pub fn generate_id(
        env: &Env,
        issuer: &Address,
        subject: &Address,
        claim_type: &String,
        timestamp: u64,
    ) -> String {
        use soroban_sdk::Bytes;
        // Strkeys for both account (G...) and contract (C...) addresses are
        // always 56 ASCII characters. Copy them into fixed-size buffers.
        let mut issuer_buf = [0u8; 56];
        let mut subject_buf = [0u8; 56];
        issuer.to_string().copy_into_slice(&mut issuer_buf);
        subject.to_string().copy_into_slice(&mut subject_buf);

        // Copy claim_type bytes into a fixed-size buffer (max 128 bytes).
        let claim_len = claim_type.len() as usize;
        let mut claim_buf = [0u8; 128];
        claim_type.copy_into_slice(&mut claim_buf[..claim_len]);

        let mut buf = Bytes::new(env);
        buf.append(&Bytes::from_slice(env, &issuer_buf));
        buf.append(&Bytes::from_slice(env, &subject_buf));
        buf.append(&Bytes::from_slice(env, &claim_buf[..claim_len]));
        buf.append(&Bytes::from_slice(env, &timestamp.to_be_bytes()));

        let hash = env.crypto().sha256(&buf);
        let hash_arr = hash.to_array();

        // Hex-encode the first 16 bytes → 32 ASCII characters.
        // This produces a valid UTF-8 string suitable for soroban_sdk::String.
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut hex_bytes = [0u8; 32];
        for i in 0..16 {
            hex_bytes[i * 2]     = HEX[(hash_arr[i] >> 4) as usize];
            hex_bytes[i * 2 + 1] = HEX[(hash_arr[i] & 0x0f) as usize];
        }
        // SAFETY: hex_bytes contains only ASCII hex digits, so it is valid UTF-8.
        String::from_str(env, core::str::from_utf8(&hex_bytes).unwrap_or(""))
    }

    /// Compute the current [`AttestationStatus`] given `current_time`.
    ///
    /// Revocation takes precedence: a revoked attestation always returns
    /// [`AttestationStatus::Revoked`] regardless of its expiration field.
    ///
    /// # Parameters
    /// - `current_time` — current ledger timestamp in seconds.
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
