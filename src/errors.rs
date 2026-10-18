use soroban_sdk::contracterror;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracterror]
#[repr(u32)]
pub enum Error {
    NotAdmin = 1,
    AdminAlreadyInitialized = 2,
    RewardBelowMinimum = 3,
    NoPendingOwner = 4,
    NotPendingOwner = 5,
    OwnerCannotBeAssignee = 6,
}
