use soroban_sdk::{contracttype, Address, BytesN, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    BountyCount,
    Bounty(BytesN<32>),
    Contributor(Address),
    Admin,
    PendingAdmin,
    MinReward(Address),
    Paused,
    FeeBps,
    MinReputation,
    ClaimWindow,
    Verifier,
    CreatorBounties(Address),
}

/// Snapshot of contract-wide settings, returned by `get_config`.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct ContractConfig {
    /// Current admin address, if initialized.
    pub admin: Option<Address>,
    /// Address that has been proposed as the next admin but has not yet
    /// accepted, if a transfer is in progress.
    pub pending_admin: Option<Address>,
    /// Whether mutating entrypoints are currently paused.
    pub paused: bool,
    /// Protocol fee in basis points (1/100th of a percent).
    pub fee_bps: u32,
    /// Minimum contributor reputation required where enforced.
    pub min_reputation: u32,
    /// Window, in ledger seconds, allowed to claim a completed bounty's reward.
    pub claim_window: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Bounty {
    pub creator: Address,
    pub title: Symbol,
    pub description: Symbol,
    pub reward_amount: i128,
    pub reward_token: Address,
    pub assignee: Option<Address>,
    pub status: Symbol,
    /// Address proposed to take over ownership of this bounty via
    /// `transfer_bounty_ownership`, pending their acceptance.
    pub pending_owner: Option<Address>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Contributor {
    pub address: Address,
    pub reputation: u32,
    pub total_earned: i128,
    pub contribution_count: u32,
}
