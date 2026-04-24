use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

use crate::types::{Attestation, IssuerTier};

// ---------------------------------------------------------------------------
// Event topic constants
// All event topic symbols are defined here as named constants so they are
// never scattered as raw literals across the codebase.
// ---------------------------------------------------------------------------

/// Admin initialized the contract.
const TOPIC_ADMIN_INIT: Symbol = symbol_short!("adm_init");
/// Attestation created.
const TOPIC_CREATED: Symbol = symbol_short!("created");
/// Attestation imported from an external source.
const TOPIC_IMPORTED: Symbol = symbol_short!("imported");
/// Attestation bridged from another chain.
const TOPIC_BRIDGED: Symbol = symbol_short!("bridged");
/// Attestation revoked.
const TOPIC_REVOKED: Symbol = symbol_short!("revoked");
/// Attestation renewed (expiration updated by issuer).
const TOPIC_RENEWED: Symbol = symbol_short!("renewed");
/// Attestation expiration updated.
const TOPIC_UPDATED: Symbol = symbol_short!("updated");
/// Attestation expired (emitted lazily on status check).
const TOPIC_EXPIRED: Symbol = symbol_short!("expired");
/// Subject requested GDPR deletion of an attestation.
const TOPIC_DEL_REQ: Symbol = symbol_short!("del_req");
/// Issuer registered.
const TOPIC_ISS_REG: Symbol = symbol_short!("iss_reg");
/// Issuer tier updated.
const TOPIC_ISS_TIER: Symbol = symbol_short!("iss_tier");
/// Issuer removed.
const TOPIC_ISS_REM: Symbol = symbol_short!("iss_rem");
/// Claim type registered.
const TOPIC_CLM_TYPE: Symbol = symbol_short!("clmtype");
/// Multi-sig proposal created.
const TOPIC_MS_PROP: Symbol = symbol_short!("ms_prop");
/// Multi-sig proposal co-signed.
const TOPIC_MS_SIGN: Symbol = symbol_short!("ms_sign");
/// Multi-sig proposal activated (threshold reached).
const TOPIC_MS_ACTV: Symbol = symbol_short!("ms_actv");
/// Admin rights transferred.
const TOPIC_ADM_XFER: Symbol = symbol_short!("adm_xfer");
/// Admin added to council.
const TOPIC_ADM_ADD: Symbol = symbol_short!("adm_add");
/// Admin removed from council.
const TOPIC_ADM_REM: Symbol = symbol_short!("adm_rem");
/// Attestation endorsed by a registered issuer.
const TOPIC_ENDORSED: Symbol = symbol_short!("endorsed");
/// Expiration hook triggered for a subject.
const TOPIC_EXP_HOOK: Symbol = symbol_short!("exp_hook");
/// Contract paused.
const TOPIC_PAUSED: Symbol = symbol_short!("paused");
/// Contract unpaused.
const TOPIC_UNPAUSED: Symbol = symbol_short!("unpaused");
/// Subject submitted an attestation request.
const TOPIC_REQ: Symbol = symbol_short!("req");
/// Issuer fulfilled an attestation request.
const TOPIC_REQ_OK: Symbol = symbol_short!("req_ok");
/// Issuer rejected an attestation request.
const TOPIC_REQ_NO: Symbol = symbol_short!("req_no");
/// Delegation created from issuer to sub-issuer.
const TOPIC_DEL_CREATED: Symbol = symbol_short!("dlg_new");
/// Delegation revoked.
const TOPIC_DEL_REVOKED: Symbol = symbol_short!("dlg_rev");
/// Whitelist mode enabled for an issuer.
const TOPIC_WL_ON: Symbol = symbol_short!("wl_on");
/// Subject added to issuer whitelist.
const TOPIC_WL_ADD: Symbol = symbol_short!("wl_add");
/// Subject removed from issuer whitelist.
const TOPIC_WL_REM: Symbol = symbol_short!("wl_rem");

pub struct Events;

impl Events {
    pub fn admin_initialized(env: &Env, admin: &Address, timestamp: u64) {
        env.events()
            .publish((TOPIC_ADMIN_INIT,), (admin.clone(), timestamp));
    }

    pub fn attestation_created(env: &Env, attestation: &Attestation) {
        env.events().publish(
            (TOPIC_CREATED, attestation.subject.clone()),
            (
                attestation.id.clone(),
                attestation.issuer.clone(),
                attestation.claim_type.clone(),
                attestation.timestamp,
                attestation.metadata.clone(),
            ),
        );
    }

    pub fn attestation_imported(env: &Env, attestation: &Attestation) {
        env.events().publish(
            (TOPIC_IMPORTED, attestation.subject.clone()),
            (
                attestation.id.clone(),
                attestation.issuer.clone(),
                attestation.claim_type.clone(),
                attestation.timestamp,
                attestation.expiration,
            ),
        );
    }

    pub fn attestation_bridged(env: &Env, attestation: &Attestation) {
        env.events().publish(
            (TOPIC_BRIDGED, attestation.subject.clone()),
            (
                attestation.id.clone(),
                attestation.issuer.clone(),
                attestation.claim_type.clone(),
                attestation
                    .source_chain
                    .clone()
                    .unwrap_or(String::from_str(env, "")),
                attestation
                    .source_tx
                    .clone()
                    .unwrap_or(String::from_str(env, "")),
            ),
        );
    }

    pub fn attestation_revoked(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        reason: &Option<String>,
    ) {
        env.events().publish(
            (TOPIC_REVOKED, issuer.clone()),
            (attestation_id.clone(), reason.clone()),
        );
    }

    pub fn attestation_revoked_with_reason(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        reason: &Option<String>,
    ) {
        env.events().publish(
            (TOPIC_REVOKED, issuer.clone()),
            (attestation_id.clone(), reason.clone()),
        );
    }

    pub fn attestation_renewed(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        new_expiration: Option<u64>,
    ) {
        env.events().publish(
            (TOPIC_RENEWED, issuer.clone()),
            (attestation_id.clone(), new_expiration),
        );
    }

    pub fn attestation_updated(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        new_expiration: Option<u64>,
    ) {
        env.events().publish(
            (TOPIC_UPDATED, issuer.clone()),
            (attestation_id.clone(), new_expiration),
        );
    }

    pub fn attestation_expired(env: &Env, attestation_id: &String, subject: &Address) {
        env.events().publish(
            (TOPIC_EXPIRED, subject.clone()),
            attestation_id.clone(),
        );
    }

    pub fn deletion_requested(
        env: &Env,
        subject: &Address,
        attestation_id: &String,
        timestamp: u64,
    ) {
        env.events().publish(
            (TOPIC_DEL_REQ, subject.clone()),
            (attestation_id.clone(), timestamp),
        );
    }

    pub fn issuer_registered(env: &Env, issuer: &Address, admin: &Address, timestamp: u64) {
        env.events().publish(
            (TOPIC_ISS_REG, issuer.clone()),
            (admin.clone(), timestamp),
        );
    }

    /// Emitted when an issuer's tier is set or updated by the admin.
    pub fn issuer_tier_updated(env: &Env, issuer: &Address, tier: &IssuerTier) {
        env.events()
            .publish((TOPIC_ISS_TIER, issuer.clone()), *tier);
    }

    pub fn issuer_removed(env: &Env, issuer: &Address, admin: &Address, timestamp: u64) {
        env.events().publish(
            (TOPIC_ISS_REM, issuer.clone()),
            (admin.clone(), timestamp),
        );
    }

    pub fn claim_type_registered(env: &Env, claim_type: &String, description: &String) {
        env.events().publish(
            (TOPIC_CLM_TYPE, claim_type.clone()),
            description.clone(),
        );
    }

    /// Emitted when a new multi-sig proposal is created.
    pub fn multisig_proposed(
        env: &Env,
        proposal_id: &String,
        proposer: &Address,
        subject: &Address,
        threshold: u32,
    ) {
        env.events().publish(
            (TOPIC_MS_PROP, subject.clone()),
            (proposal_id.clone(), proposer.clone(), threshold),
        );
    }

    /// Emitted when an issuer co-signs a multi-sig proposal.
    pub fn multisig_cosigned(
        env: &Env,
        proposal_id: &String,
        signer: &Address,
        signatures_so_far: u32,
        threshold: u32,
    ) {
        env.events().publish(
            (TOPIC_MS_SIGN, signer.clone()),
            (proposal_id.clone(), signatures_so_far, threshold),
        );
    }

    /// Emitted when a multi-sig proposal reaches threshold and the attestation is activated.
    pub fn multisig_activated(env: &Env, proposal_id: &String, attestation_id: &String) {
        env.events().publish(
            (TOPIC_MS_ACTV,),
            (proposal_id.clone(), attestation_id.clone()),
        );
    }

    /// Emitted when admin rights are transferred to a new address.
    pub fn admin_transferred(env: &Env, old_admin: &Address, new_admin: &Address) {
        env.events().publish(
            (TOPIC_ADM_XFER,),
            (old_admin.clone(), new_admin.clone()),
        );
    }

    /// Emitted when an admin adds a new admin to the council.
    pub fn admin_added(env: &Env, by_admin: &Address, new_admin: &Address, timestamp: u64) {
        env.events().publish(
            (TOPIC_ADM_ADD, by_admin.clone()),
            (new_admin.clone(), timestamp),
        );
    }

    /// Emitted when an admin removes an admin from the council.
    pub fn admin_removed(env: &Env, by_admin: &Address, removed_admin: &Address, timestamp: u64) {
        env.events().publish(
            (TOPIC_ADM_REM, by_admin.clone()),
            (removed_admin.clone(), timestamp),
        );
    }

    /// Emitted when a registered issuer endorses an existing attestation.
    pub fn attestation_endorsed(
        env: &Env,
        attestation_id: &String,
        endorser: &Address,
        timestamp: u64,
    ) {
        env.events().publish(
            (TOPIC_ENDORSED, endorser.clone()),
            (attestation_id.clone(), timestamp),
        );
    }

    /// Emitted when an expiration hook is triggered for a subject's attestation.
    pub fn expiration_hook_triggered(
        env: &Env,
        subject: &Address,
        attestation_id: &String,
        expiration: u64,
    ) {
        env.events().publish(
            (TOPIC_EXP_HOOK, subject.clone()),
            (attestation_id.clone(), expiration),
        );
    }

    /// Emitted when the admin pauses the contract.
    pub fn contract_paused(env: &Env, admin: &Address, timestamp: u64) {
        env.events()
            .publish((TOPIC_PAUSED,), (admin.clone(), timestamp));
    }

    /// Emitted when the admin unpauses the contract.
    pub fn contract_unpaused(env: &Env, admin: &Address, timestamp: u64) {
        env.events()
            .publish((TOPIC_UNPAUSED,), (admin.clone(), timestamp));
    }

    /// Emitted when a subject submits an attestation request to an issuer.
    pub fn attestation_requested(
        env: &Env,
        request_id: &String,
        subject: &Address,
        issuer: &Address,
        claim_type: &String,
        expires_at: u64,
    ) {
        env.events().publish(
            (TOPIC_REQ, issuer.clone()),
            (
                request_id.clone(),
                subject.clone(),
                claim_type.clone(),
                expires_at,
            ),
        );
    }

    /// Emitted when an issuer fulfills an attestation request.
    pub fn request_fulfilled(
        env: &Env,
        request_id: &String,
        issuer: &Address,
        attestation_id: &String,
    ) {
        env.events().publish(
            (TOPIC_REQ_OK, issuer.clone()),
            (request_id.clone(), attestation_id.clone()),
        );
    }

    /// Emitted when an issuer rejects an attestation request.
    pub fn request_rejected(
        env: &Env,
        request_id: &String,
        issuer: &Address,
        reason: &Option<String>,
    ) {
        env.events().publish(
            (TOPIC_REQ_NO, issuer.clone()),
            (request_id.clone(), reason.clone()),
        );
    }

    /// Emitted when issuer creates a delegation to a sub-issuer for a claim type.
    pub fn delegation_created(
        env: &Env,
        delegator: &Address,
        delegate: &Address,
        claim_type: &String,
        expiration: Option<u64>,
    ) {
        env.events().publish(
            (TOPIC_DEL_CREATED, delegator.clone()),
            (delegate.clone(), claim_type.clone(), expiration),
        );
    }

    /// Emitted when issuer revokes a delegation.
    pub fn delegation_revoked(
        env: &Env,
        delegator: &Address,
        delegate: &Address,
        claim_type: &String,
    ) {
        env.events().publish(
            (TOPIC_DEL_REVOKED, delegator.clone()),
            (delegate.clone(), claim_type.clone()),
        );
    }

    pub fn whitelist_mode_enabled(env: &Env, issuer: &Address) {
        env.events()
            .publish((TOPIC_WL_ON, issuer.clone()), ());
    }

    pub fn whitelist_updated(env: &Env, issuer: &Address, subject: &Address, added: bool) {
        let sym = if added { TOPIC_WL_ADD } else { TOPIC_WL_REM };
        env.events().publish((sym, issuer.clone()), subject.clone());
    }
}
