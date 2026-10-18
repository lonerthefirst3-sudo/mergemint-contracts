use soroban_sdk::{Address, BytesN, Env, Vec};

use crate::errors::Error;
use crate::types::{Bounty, ContractConfig, Contributor, DataKey};

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}

/// Panics with `Error::NotAdmin` unless `caller` is the configured admin and
/// has authorized this invocation.
pub fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
    caller.require_auth();
    match get_admin(env) {
        Some(admin) if admin == *caller => Ok(()),
        _ => Err(Error::NotAdmin),
    }
}

pub fn get_min_reward(env: &Env, token: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::MinReward(token.clone()))
        .unwrap_or(0)
}

pub fn set_min_reward(env: &Env, token: &Address, amount: &i128) {
    env.storage()
        .persistent()
        .set(&DataKey::MinReward(token.clone()), amount);
}

pub fn get_pending_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::PendingAdmin)
}

pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::PendingAdmin, pending);
}

pub fn clear_pending_admin(env: &Env) {
    env.storage().persistent().remove(&DataKey::PendingAdmin);
}

pub fn get_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

pub fn set_paused(env: &Env, paused: &bool) {
    env.storage().persistent().set(&DataKey::Paused, paused);
}

pub fn get_fee_bps(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::FeeBps)
        .unwrap_or(0)
}

pub fn set_fee_bps(env: &Env, fee_bps: &u32) {
    env.storage().persistent().set(&DataKey::FeeBps, fee_bps);
}

pub fn get_min_reputation(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::MinReputation)
        .unwrap_or(0)
}

pub fn set_min_reputation(env: &Env, min_reputation: &u32) {
    env.storage()
        .persistent()
        .set(&DataKey::MinReputation, min_reputation);
}

pub fn get_claim_window(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::ClaimWindow)
        .unwrap_or(0)
}

pub fn set_claim_window(env: &Env, claim_window: &u64) {
    env.storage()
        .persistent()
        .set(&DataKey::ClaimWindow, claim_window);
}

pub fn get_verifier(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::Verifier)
}

pub fn set_verifier(env: &Env, verifier: &Address) {
    env.storage().persistent().set(&DataKey::Verifier, verifier);
}

pub fn get_creator_bounties(env: &Env, creator: &Address) -> Vec<BytesN<32>> {
    env.storage()
        .persistent()
        .get(&DataKey::CreatorBounties(creator.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn add_creator_bounty(env: &Env, creator: &Address, id: &BytesN<32>) {
    let mut list = get_creator_bounties(env, creator);
    list.push_back(id.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CreatorBounties(creator.clone()), &list);
}

pub fn remove_creator_bounty(env: &Env, creator: &Address, id: &BytesN<32>) {
    let list = get_creator_bounties(env, creator);
    let mut new_list = Vec::new(env);
    for existing in list.iter() {
        if existing != *id {
            new_list.push_back(existing);
        }
    }
    env.storage()
        .persistent()
        .set(&DataKey::CreatorBounties(creator.clone()), &new_list);
}

pub fn get_config(env: &Env) -> ContractConfig {
    ContractConfig {
        admin: get_admin(env),
        pending_admin: get_pending_admin(env),
        paused: get_paused(env),
        fee_bps: get_fee_bps(env),
        min_reputation: get_min_reputation(env),
        claim_window: get_claim_window(env),
    }
}

pub fn get_bounty_count(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::BountyCount)
        .unwrap_or(0)
}

pub fn set_bounty_count(env: &Env, count: &u64) {
    env.storage()
        .persistent()
        .set(&DataKey::BountyCount, count);
}

pub fn store_bounty(env: &Env, id: &BytesN<32>, bounty: &Bounty) {
    env.storage()
        .persistent()
        .set(&DataKey::Bounty(id.clone()), bounty);
}

pub fn get_bounty(env: &Env, id: &BytesN<32>) -> Option<Bounty> {
    env.storage()
        .persistent()
        .get(&DataKey::Bounty(id.clone()))
}

pub fn store_contributor(env: &Env, address: &Address, contributor: &Contributor) {
    env.storage()
        .persistent()
        .set(&DataKey::Contributor(address.clone()), contributor);
}

pub fn get_contributor(env: &Env, address: &Address) -> Option<Contributor> {
    env.storage()
        .persistent()
        .get(&DataKey::Contributor(address.clone()))
}
