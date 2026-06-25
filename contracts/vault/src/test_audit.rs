#![cfg(test)]

use super::*;
use crate::types::{AuditAction, RetryConfig, ThresholdStrategy, VelocityConfig};
use crate::{InitConfig, VaultDAO, VaultDAOClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Env, Symbol, Vec,
};

fn make_audit_config(env: &Env, signers: Vec<Address>, threshold: u32) -> InitConfig {
    InitConfig {
        signers,
        threshold,
        quorum: 0,
        spending_limit: 1000,
        daily_limit: 5000,
        weekly_limit: 10000,
        timelock_threshold: 500,
        timelock_delay: 100,
        velocity_limit: VelocityConfig {
            limit: 100,
            window: 3600,
        },
        threshold_strategy: ThresholdStrategy::Fixed,
        default_voting_deadline: 0,
        veto_addresses: Vec::new(env),
        retry_config: RetryConfig {
            enabled: false,
            max_retries: 0,
            initial_backoff_ledgers: 0,
        },
        recovery_config: crate::types::RecoveryConfig::default(env),
        staking_config: types::StakingConfig::default(),
        pre_execution_hooks: Vec::new(env),
        post_execution_hooks: Vec::new(env),
    }
}

#[test]
fn test_audit_trail_creation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VaultDAO, ());
    let client = VaultDAOClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);

    let mut signers = Vec::new(&env);
    signers.push_back(admin.clone());
    signers.push_back(signer1.clone());

    let config = make_audit_config(&env, signers, 1);
    client.initialize(&admin, &config);

    // Verify initialization audit entry
    let audit_entry = client.get_audit_entry(&1);
    assert_eq!(audit_entry.id, 1);
    assert_eq!(audit_entry.action, AuditAction::Initialize);
    assert_eq!(audit_entry.actor, admin);
    assert_eq!(audit_entry.prev_hash, 0);

    // Set role and verify audit
    client.set_role(&admin, &signer1, &Role::Treasurer);
    let audit_entry2 = client.get_audit_entry(&2);
    assert_eq!(audit_entry2.action, AuditAction::SetRole);
    assert_eq!(audit_entry2.prev_hash, audit_entry.hash);
}

#[test]
fn test_audit_trail_hash_chain() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VaultDAO, ());
    let client = VaultDAOClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let user = Address::generate(&env);
    let token = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token);
    token_client.mint(&contract_id, &10000);

    let mut signers = Vec::new(&env);
    signers.push_back(admin.clone());
    signers.push_back(signer1.clone());

    let config = make_audit_config(&env, signers, 1);
    client.initialize(&admin, &config);
    client.set_role(&admin, &signer1, &Role::Treasurer);

    let proposal_id = client.propose_transfer(
        &signer1,
        &user,
        &token,
        &100,
        &Symbol::new(&env, "test"),
        &Priority::Normal,
        &Vec::new(&env),
        &ConditionLogic::And,
        &0i128,
    );
    client.approve_proposal(&signer1, &proposal_id);

    // Verify hash chain integrity
    let entry1 = client.get_audit_entry(&1);
    let entry2 = client.get_audit_entry(&2);
    let entry3 = client.get_audit_entry(&3);
    let entry4 = client.get_audit_entry(&4);

    assert_eq!(entry2.prev_hash, entry1.hash);
    assert_eq!(entry3.prev_hash, entry2.hash);
    assert_eq!(entry4.prev_hash, entry3.hash);
}

#[test]
fn test_audit_trail_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VaultDAO, ());
    let client = VaultDAOClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let user = Address::generate(&env);

    let mut signers = Vec::new(&env);
    signers.push_back(admin.clone());
    signers.push_back(signer1.clone());

    let config = make_audit_config(&env, signers, 1);
    client.initialize(&admin, &config);
    client.set_role(&admin, &signer1, &Role::Treasurer);
    client.add_signer(&admin, &user);

    // Verify entire audit trail
    let is_valid = client.verify_audit_trail(&1, &3);
    assert_eq!(is_valid, true);
}

#[test]
fn test_audit_trail_all_actions() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VaultDAO, ());
    let client = VaultDAOClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let user = Address::generate(&env);
    let token = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token);
    token_client.mint(&contract_id, &10000);

    let mut signers = Vec::new(&env);
    signers.push_back(admin.clone());
    signers.push_back(signer1.clone());

    let config = make_audit_config(&env, signers, 1);
    client.initialize(&admin, &config);
    client.set_role(&admin, &signer1, &Role::Treasurer);
    client.add_signer(&admin, &signer2);

    let proposal_id = client.propose_transfer(
        &signer1,
        &user,
        &token,
        &100,
        &Symbol::new(&env, "test"),
        &Priority::Normal,
        &Vec::new(&env),
        &ConditionLogic::And,
        &0i128,
    );
    client.approve_proposal(&signer1, &proposal_id);

    // Verify all audit entries exist
    let entry1 = client.get_audit_entry(&1);
    assert_eq!(entry1.action, AuditAction::Initialize);

    let entry2 = client.get_audit_entry(&2);
    assert_eq!(entry2.action, AuditAction::SetRole);

    let entry3 = client.get_audit_entry(&3);
    assert_eq!(entry3.action, AuditAction::AddSigner);

    let entry4 = client.get_audit_entry(&4);
    assert_eq!(entry4.action, AuditAction::ProposeTransfer);

    let entry5 = client.get_audit_entry(&5);
    assert_eq!(entry5.action, AuditAction::ApproveProposal);

    // Verify entire chain
    let is_valid = client.verify_audit_trail(&1, &5);
    assert_eq!(is_valid, true);
}
