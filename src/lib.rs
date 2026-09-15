//! soroban-milestone-escrow
//!
//! A minimal milestone-based escrow contract for Soroban.
//!
//! A `client` deposits funds up front, split across a list of milestones.
//! The `client` approves each milestone as work is delivered, which
//! releases that milestone's funds to the `provider`. If the client and
//! provider disagree, either side can raise a dispute and a designated
//! `arbiter` resolves it.
//!
//! This is a small building block for freelance/contract-work payment
//! flows in the Stellar ecosystem (a space that already has related
//! projects like Trustless Work and SafeTrust) — not a replacement for
//! them, but a compact reference implementation focused on readability.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum MilestoneStatus {
    Pending,
    Approved,
    Released,
    Disputed,
}

#[contracttype]
#[derive(Clone)]
pub struct Milestone {
    pub description: String,
    pub amount: i128,
    pub status: MilestoneStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub client: Address,
    pub provider: Address,
    pub arbiter: Address,
    pub token: Address,
    pub milestones: Vec<Milestone>,
}

#[contracttype]
pub enum DataKey {
    Escrow(u64),
    NextId,
}

#[contract]
pub struct MilestoneEscrowContract;

#[contractimpl]
impl MilestoneEscrowContract {
    /// Create a new escrow. Pulls the sum of all milestone amounts from
    /// `client` into the contract up front.
    pub fn create_escrow(
        env: Env,
        client: Address,
        provider: Address,
        arbiter: Address,
        token: Address,
        milestones: Vec<Milestone>,
    ) -> u64 {
        client.require_auth();
        assert!(milestones.len() > 0, "at least one milestone is required");

        let mut total: i128 = 0;
        for m in milestones.iter() {
            assert!(m.amount > 0, "milestone amount must be positive");
            assert_eq!(m.status, MilestoneStatus::Pending, "new milestones must start Pending");
            total += m.amount;
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&client, &env.current_contract_address(), &total);

        let id = Self::next_id(&env);
        let escrow = Escrow {
            client,
            provider,
            arbiter,
            token,
            milestones,
        };
        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
        id
    }

    /// Client approves a milestone as complete, making it releasable.
    pub fn approve_milestone(env: Env, id: u64, index: u32) {
        let mut escrow = Self::load(&env, id);
        escrow.client.require_auth();

        let mut m = escrow.milestones.get(index).expect("milestone not found");
        assert_eq!(m.status, MilestoneStatus::Pending, "milestone must be Pending to approve");
        m.status = MilestoneStatus::Approved;
        escrow.milestones.set(index, m);
        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
    }

    /// Releases an approved milestone's funds to the provider. Callable
    /// by anyone once approved (no auth needed beyond the prior approval),
    /// since the outcome is already determined.
    pub fn release_milestone(env: Env, id: u64, index: u32) {
        let mut escrow = Self::load(&env, id);
        let m = escrow.milestones.get(index).expect("milestone not found");
        assert_eq!(m.status, MilestoneStatus::Approved, "milestone must be Approved to release");

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&env.current_contract_address(), &escrow.provider, &m.amount);

        let mut updated = m;
        updated.status = MilestoneStatus::Released;
        escrow.milestones.set(index, updated);
        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
    }

    /// Provider disputes a milestone (e.g. client is refusing to approve
    /// work the provider believes is complete).
    pub fn raise_dispute(env: Env, id: u64, index: u32) {
        let mut escrow = Self::load(&env, id);
        escrow.provider.require_auth();

        let mut m = escrow.milestones.get(index).expect("milestone not found");
        assert_eq!(m.status, MilestoneStatus::Pending, "only a Pending milestone can be disputed");
        m.status = MilestoneStatus::Disputed;
        escrow.milestones.set(index, m);
        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
    }

    /// Arbiter resolves a disputed milestone, sending funds to whichever
    /// side the arbiter decides.
    pub fn resolve_dispute(env: Env, id: u64, index: u32, release_to_provider: bool) {
        let mut escrow = Self::load(&env, id);
        escrow.arbiter.require_auth();

        let m = escrow.milestones.get(index).expect("milestone not found");
        assert_eq!(m.status, MilestoneStatus::Disputed, "milestone must be Disputed to resolve");

        let destination = if release_to_provider {
            escrow.provider.clone()
        } else {
            escrow.client.clone()
        };
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&env.current_contract_address(), &destination, &m.amount);

        let mut updated = m;
        updated.status = MilestoneStatus::Released;
        escrow.milestones.set(index, updated);
        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
    }

    /// Read-only view of an escrow's current state.
    pub fn get_escrow(env: Env, id: u64) -> Escrow {
        Self::load(&env, id)
    }

    fn load(env: &Env, id: u64) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(id))
            .expect("escrow not found")
    }

    fn next_id(env: &Env) -> u64 {
        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(0);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));
        id
    }
}

mod test;
