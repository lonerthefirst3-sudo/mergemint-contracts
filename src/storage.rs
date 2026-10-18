use soroban_sdk::{Address, BytesN, Env};

use crate::errors::Error;
use crate::types::{Bounty, Contributor, DataKey};

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
