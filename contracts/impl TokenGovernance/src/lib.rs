//! Simple Token-Based Governance Smart Contract

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map};

#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub description: Symbol,
    pub votes_for: u32,
    pub votes_against: u32,
    pub executed: bool,
}

#[contracttype]
pub enum GovernanceKey {
    Proposals,
    NextProposalId,
    TokenHolders,
    Votes,
}

#[contract]
pub struct TokenGovernance;

#[contractimpl]
impl TokenGovernance {
    pub fn init(env: Env) {
        env.storage().instance().set(&GovernanceKey::Proposals, &Vec::<Proposal>::new(&env));
        env.storage().instance().set(&GovernanceKey::NextProposalId, &0u32);
        env.storage().instance().set(&GovernanceKey::TokenHolders, &Map::<Address, u32>::new(&env));
        env.storage().instance().set(&GovernanceKey::Votes, &Map::<(u32, Address), bool>::new(&env));
    }

    pub fn add_holder(env: Env, addr: Address, tokens: u32) {
        let mut holders: Map<Address, u32> = env.storage().instance().get(&GovernanceKey::TokenHolders).unwrap();
        holders.set(addr, tokens);
        env.storage().instance().set(&GovernanceKey::TokenHolders, &holders);
    }

    pub fn create_proposal(env: Env, description: Symbol) {
        let mut proposals: Vec<Proposal> = env.storage().instance().get(&GovernanceKey::Proposals).unwrap();
        let mut next_id: u32 = env.storage().instance().get(&GovernanceKey::NextProposalId).unwrap();

        proposals.push_back(Proposal {
            id: next_id,
            description,
            votes_for: 0,
            votes_against: 0,
            executed: false,
        });

        next_id += 1;
        env.storage().instance().set(&GovernanceKey::Proposals, &proposals);
        env.storage().instance().set(&GovernanceKey::NextProposalId, &next_id);
    }

    pub fn vote(env: Env, proposal_id: u32, voter: Address, support: bool) {
        let mut proposals: Vec<Proposal> = env.storage().instance().get(&GovernanceKey::Proposals).unwrap();
        let mut votes: Map<(u32, Address), bool> = env.storage().instance().get(&GovernanceKey::Votes).unwrap();
        let holders: Map<Address, u32> = env.storage().instance().get(&GovernanceKey::TokenHolders).unwrap();

        let key = (proposal_id, voter.clone());
        if votes.contains_key(key.clone()) {
            panic!("Already voted on this proposal");
        }

        let voter_tokens = holders.get(voter.clone()).unwrap_or(0);
        if voter_tokens == 0 {
            panic!("Only token holders can vote");
        }

        let mut proposal = proposals.get(proposal_id).expect("Invalid proposal ID");

        if support {
            proposal.votes_for += voter_tokens;
        } else {
            proposal.votes_against += voter_tokens;
        }

        votes.set(key, true);
        proposals.set(proposal_id, proposal);

        env.storage().instance().set(&GovernanceKey::Proposals, &proposals);
        env.storage().instance().set(&GovernanceKey::Votes, &votes);
    }

    pub fn get_proposals(env: Env) -> Vec<Proposal> {
        env.storage().instance().get(&GovernanceKey::Proposals).unwrap()
    }

    pub fn get_token_balance(env: Env, addr: Address) -> u32 {
        let holders: Map<Address, u32> = env.storage().instance().get(&GovernanceKey::TokenHolders).unwrap();
        holders.get(addr).unwrap_or(0)
    }
}
