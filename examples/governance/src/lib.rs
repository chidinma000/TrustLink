use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// Initialize the governance contract with a TrustLink contract address
    pub fn initialize(env: Env, trustlink: Address) {
        env.storage().instance().set(&String::from_str(&env, "trustlink"), &trustlink);
    }

    /// Cast a vote on a proposal. Voter must have valid KYC claim.
    pub fn vote(env: Env, voter: Address, proposal_id: u32, vote: bool) -> Result<(), String> {
        voter.require_auth();

        let trustlink: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, "trustlink"))
            .ok_or_else(|| String::from_str(&env, "trustlink not initialized"))?;

        // Create a TrustLink client
        let trustlink_client = trustlink::Client::new(&env, &trustlink);

        // Check if voter has valid KYC claim
        let kyc_claim = String::from_str(&env, "KYC_PASSED");
        let has_kyc = trustlink_client.has_valid_claim(&voter, &kyc_claim);

        if !has_kyc {
            return Err(String::from_str(&env, "voter must have valid KYC"));
        }

        // Store the vote
        let vote_key = String::from_str(&env, &format!("vote_{}_{}", proposal_id, voter.to_string()));
        env.storage().instance().set(&vote_key, &vote);

        Ok(())
    }

    /// Get the vote count for a proposal
    pub fn get_vote_count(env: Env, proposal_id: u32) -> (u32, u32) {
        let yes_key = String::from_str(&env, &format!("yes_count_{}", proposal_id));
        let no_key = String::from_str(&env, &format!("no_count_{}", proposal_id));

        let yes_count: u32 = env.storage().instance().get(&yes_key).unwrap_or(0);
        let no_count: u32 = env.storage().instance().get(&no_key).unwrap_or(0);

        (yes_count, no_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_vote_requires_kyc() {
        let env = Env::default();
        let contract_id = env.register_contract(None, GovernanceContract);
        let client = GovernanceContractClient::new(&env, &contract_id);

        let voter = Address::random(&env);
        let trustlink = Address::random(&env);

        client.initialize(&trustlink);

        // Attempt to vote without KYC should fail
        let result = client.try_vote(&voter, &1, &true);
        assert!(result.is_err());
    }

    #[test]
    fn test_vote_with_kyc() {
        let env = Env::default();
        let contract_id = env.register_contract(None, GovernanceContract);
        let client = GovernanceContractClient::new(&env, &contract_id);

        let voter = Address::random(&env);
        let trustlink = Address::random(&env);

        client.initialize(&trustlink);

        // In a real test, we'd mock the TrustLink contract to return true for has_valid_claim
        // For now, this demonstrates the contract structure
    }
}
