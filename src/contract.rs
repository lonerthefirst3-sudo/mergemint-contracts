use soroban_sdk::{
    contract, contractimpl, panic_with_error, token::TokenClient, Address, BytesN, Env, Symbol,
};

use crate::errors::Error;
use crate::events;
use crate::storage;
use crate::types::{Bounty, ContractConfig, Contributor};

const STATUS_OPEN: &str = "open";
const STATUS_IN_PROGRESS: &str = "in_progress";

fn generate_bounty_id(env: &Env) -> BytesN<32> {
    let count = storage::get_bounty_count(env);
    let mut buf = [0u8; 32];
    let count_bytes = count.to_be_bytes();
    buf[24..32].copy_from_slice(&count_bytes);
    BytesN::from_array(env, &buf)
}

#[contract]
pub struct MergeMintContract;

#[contractimpl]
impl MergeMintContract {
    pub fn create_bounty(
        env: Env,
        creator: Address,
        title: Symbol,
        description: Symbol,
        reward_amount: i128,
        reward_token: Address,
    ) -> BytesN<32> {
        creator.require_auth();

        let min_reward = storage::get_min_reward(&env, &reward_token);
        if reward_amount < min_reward {
            panic_with_error(&env, Error::RewardBelowMinimum);
        }

        let count = storage::get_bounty_count(&env);
        let id = generate_bounty_id(&env);

        let bounty = Bounty {
            creator,
            title,
            description,
            reward_amount,
            reward_token,
            assignee: None,
            status: Symbol::new(&env, STATUS_OPEN),
        };

        storage::store_bounty(&env, &id, &bounty);
        storage::set_bounty_count(&env, &(count + 1));

        events::emit_bounty_created(&env, &id, &bounty.creator, &reward_amount);
        id
    }

    pub fn claim_bounty(env: Env, contributor: Address, bounty_id: BytesN<32>) {
        contributor.require_auth();

        let mut bounty = storage::get_bounty(&env, &bounty_id).expect("bounty not found");

        if bounty.assignee.is_some() {
            panic!("bounty already assigned");
        }

        bounty.assignee = Some(contributor.clone());
        bounty.status = Symbol::new(&env, STATUS_IN_PROGRESS);

        storage::store_bounty(&env, &bounty_id, &bounty);
        events::emit_bounty_claimed(&env, &bounty_id, &contributor);
    }

    pub fn complete_bounty(env: Env, verifier: Address, bounty_id: BytesN<32>) {
        verifier.require_auth();

        let bounty = storage::get_bounty(&env, &bounty_id).expect("bounty not found");
        let assignee = bounty.assignee.clone().expect("bounty has no assignee");

        let token = TokenClient::new(&env, &bounty.reward_token);
        token.transfer(&verifier, &assignee, &bounty.reward_amount);

        let mut contributor = storage::get_contributor(&env, &assignee).unwrap_or(Contributor {
            address: assignee.clone(),
            reputation: 0,
            total_earned: 0,
            contribution_count: 0,
        });

        contributor.reputation += 10;
        contributor.total_earned += bounty.reward_amount;
        contributor.contribution_count += 1;

        storage::store_contributor(&env, &assignee, &contributor);

        events::emit_bounty_completed(&env, &bounty_id, &assignee);
        events::emit_reward_paid(&env, &bounty_id, &assignee, &bounty.reward_amount);
    }

    pub fn get_bounty(env: Env, bounty_id: BytesN<32>) -> Option<Bounty> {
        storage::get_bounty(&env, &bounty_id)
    }

    pub fn get_contributor(env: Env, address: Address) -> Option<Contributor> {
        storage::get_contributor(&env, &address)
    }

    pub fn get_bounty_count(env: Env) -> u64 {
        storage::get_bounty_count(&env)
    }

    /// One-time admin bootstrap. Panics with `AdminAlreadyInitialized` if an
    /// admin is already set.
    pub fn init_admin(env: Env, admin: Address) {
        if storage::get_admin(&env).is_some() {
            panic_with_error(&env, Error::AdminAlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
    }

    /// Admin-only. Sets the minimum reward amount accepted by `create_bounty`
    /// for `token`. Defaults to 0 (no minimum) until set.
    pub fn set_min_reward(env: Env, admin: Address, token: Address, amount: i128) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        storage::set_min_reward(&env, &token, &amount);
    }

    pub fn get_min_reward(env: Env, token: Address) -> i128 {
        storage::get_min_reward(&env, &token)
    }

    /// Read-only snapshot of contract-wide settings: admin, pending admin,
    /// pause flag, fee bps, minimum reputation and claim window.
    pub fn get_config(env: Env) -> ContractConfig {
        storage::get_config(&env)
    }

    /// Admin-only. Pauses/unpauses mutating entrypoints and emits
    /// `config_changed`.
    pub fn set_paused(env: Env, admin: Address, paused: bool) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        let old = storage::get_paused(&env);
        storage::set_paused(&env, &paused);
        events::emit_config_changed_bool(&env, "paused", old, paused);
    }

    /// Admin-only. Sets the protocol fee in basis points and emits
    /// `config_changed`.
    pub fn set_fee_bps(env: Env, admin: Address, fee_bps: u32) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        let old = storage::get_fee_bps(&env);
        storage::set_fee_bps(&env, &fee_bps);
        events::emit_config_changed_u32(&env, "fee_bps", old, fee_bps);
    }

    /// Admin-only. Sets the minimum contributor reputation and emits
    /// `config_changed`.
    pub fn set_min_reputation(env: Env, admin: Address, min_reputation: u32) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        let old = storage::get_min_reputation(&env);
        storage::set_min_reputation(&env, &min_reputation);
        events::emit_config_changed_u32(&env, "min_reputation", old, min_reputation);
    }

    /// Admin-only. Sets the claim window (in ledger seconds) and emits
    /// `config_changed`.
    pub fn set_claim_window(env: Env, admin: Address, claim_window: u64) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        let old = storage::get_claim_window(&env);
        storage::set_claim_window(&env, &claim_window);
        events::emit_config_changed_u64(&env, "claim_window", old, claim_window);
    }

    /// Admin-only. Sets the verifier address and emits `config_changed`.
    pub fn set_verifier(env: Env, admin: Address, verifier: Address) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        let old = storage::get_verifier(&env);
        storage::set_verifier(&env, &verifier);
        events::emit_config_changed_address(&env, "verifier", old, verifier);
    }

    /// Admin-only. Begins a two-step admin transfer by recording
    /// `new_admin` as the pending admin. The transfer only completes once
    /// `new_admin` calls `accept_admin`.
    pub fn transfer_admin(env: Env, admin: Address, new_admin: Address) {
        if let Err(e) = storage::require_admin(&env, &admin) {
            panic_with_error(&env, e);
        }
        storage::set_pending_admin(&env, &new_admin);
    }

    /// Completes a pending admin transfer. Must be called by the pending
    /// admin. Emits `config_changed` for the `admin` key.
    pub fn accept_admin(env: Env, new_admin: Address) {
        new_admin.require_auth();
        let pending = storage::get_pending_admin(&env);
        if pending.as_ref() != Some(&new_admin) {
            panic_with_error(&env, Error::NotPendingOwner);
        }
        let old = storage::get_admin(&env);
        storage::set_admin(&env, &new_admin);
        storage::clear_pending_admin(&env);
        events::emit_config_changed_address(&env, "admin", old, new_admin);
    }
}
