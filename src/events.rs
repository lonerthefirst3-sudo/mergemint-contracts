use soroban_sdk::{Address, BytesN, Env, Symbol};

/// Common topic used by every admin configuration-change event. Each
/// variant below shares the same (config_changed, key) topic shape and
/// differs only in the data type of old/new value.
fn config_changed_topic(env: &Env, key: &str) -> (Symbol, Symbol) {
    (Symbol::new(env, "config_changed"), Symbol::new(env, key))
}

pub fn emit_config_changed_bool(env: &Env, key: &str, old_value: bool, new_value: bool) {
    env.events()
        .publish(config_changed_topic(env, key), (old_value, new_value));
}

pub fn emit_config_changed_u32(env: &Env, key: &str, old_value: u32, new_value: u32) {
    env.events()
        .publish(config_changed_topic(env, key), (old_value, new_value));
}

pub fn emit_config_changed_u64(env: &Env, key: &str, old_value: u64, new_value: u64) {
    env.events()
        .publish(config_changed_topic(env, key), (old_value, new_value));
}

pub fn emit_config_changed_address(
    env: &Env,
    key: &str,
    old_value: Option<Address>,
    new_value: Address,
) {
    env.events()
        .publish(config_changed_topic(env, key), (old_value, new_value));
}

pub fn emit_bounty_created(env: &Env, bounty_id: &BytesN<32>, creator: &Address, reward: &i128) {
    let topic = Symbol::new(env, "bounty_created");
    env.events().publish(
        (topic, creator.clone()),
        (bounty_id.clone(), *reward),
    );
}

pub fn emit_bounty_claimed(env: &Env, bounty_id: &BytesN<32>, contributor: &Address) {
    let topic = Symbol::new(env, "bounty_claimed");
    env.events().publish(
        (topic, contributor.clone()),
        bounty_id.clone(),
    );
}

pub fn emit_bounty_completed(env: &Env, bounty_id: &BytesN<32>, contributor: &Address) {
    let topic = Symbol::new(env, "bounty_completed");
    env.events().publish(
        (topic, contributor.clone()),
        bounty_id.clone(),
    );
}

pub fn emit_reward_paid(env: &Env, bounty_id: &BytesN<32>, contributor: &Address, amount: &i128) {
    let topic = Symbol::new(env, "reward_paid");
    env.events().publish(
        (topic, contributor.clone()),
        (bounty_id.clone(), *amount),
    );
}
