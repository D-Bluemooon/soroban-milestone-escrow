#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, vec, Env, String};

fn create_token<'a>(env: &Env, admin: &Address) -> (Address, token::StellarAssetClient<'a>, token::Client<'a>) {
    let contract_address = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let admin_client = token::StellarAssetClient::new(env, &contract_address);
    let client = token::Client::new(env, &contract_address);
    (contract_address, admin_client, client)
}

fn setup(env: &Env) -> (Address, Address, Address, Address, token::Client<'static>) {
    let admin = Address::generate(env);
    let client_addr = Address::generate(env);
    let provider = Address::generate(env);
    let arbiter = Address::generate(env);
    let (token_address, token_admin, token_client) = create_token(env, &admin);
    token_admin.mint(&client_addr, &10_000);
    (client_addr, provider, arbiter, token_address, token_client)
}

#[test]
fn happy_path_approve_and_release() {
    let env = Env::default();
    env.mock_all_auths();
    let (client_addr, provider, arbiter, token_address, token_client) = setup(&env);

    let contract_id = env.register_contract(None, MilestoneEscrowContract);
    let escrow_client = MilestoneEscrowContractClient::new(&env, &contract_id);

    let milestones = vec![
        &env,
        Milestone {
            description: String::from_str(&env, "Design doc"),
            amount: 300,
            status: MilestoneStatus::Pending,
        },
        Milestone {
            description: String::from_str(&env, "Implementation"),
            amount: 700,
            status: MilestoneStatus::Pending,
        },
    ];

    let id = escrow_client.create_escrow(&client_addr, &provider, &arbiter, &token_address, &milestones);

    escrow_client.approve_milestone(&id, &0);
    escrow_client.release_milestone(&id, &0);
    assert_eq!(token_client.balance(&provider), 300);

    let escrow = escrow_client.get_escrow(&id);
    assert_eq!(escrow.milestones.get(0).unwrap().status, MilestoneStatus::Released);
    assert_eq!(escrow.milestones.get(1).unwrap().status, MilestoneStatus::Pending);
}

#[test]
fn dispute_resolved_in_favor_of_provider() {
    let env = Env::default();
    env.mock_all_auths();
    let (client_addr, provider, arbiter, token_address, token_client) = setup(&env);

    let contract_id = env.register_contract(None, MilestoneEscrowContract);
    let escrow_client = MilestoneEscrowContractClient::new(&env, &contract_id);

    let milestones = vec![
        &env,
        Milestone {
            description: String::from_str(&env, "Only milestone"),
            amount: 500,
            status: MilestoneStatus::Pending,
        },
    ];

    let id = escrow_client.create_escrow(&client_addr, &provider, &arbiter, &token_address, &milestones);

    escrow_client.raise_dispute(&id, &0);
    escrow_client.resolve_dispute(&id, &0, &true);

    assert_eq!(token_client.balance(&provider), 500);
}

#[test]
#[should_panic(expected = "milestone must be Pending to approve")]
fn cannot_approve_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let (client_addr, provider, arbiter, token_address, _token_client) = setup(&env);

    let contract_id = env.register_contract(None, MilestoneEscrowContract);
    let escrow_client = MilestoneEscrowContractClient::new(&env, &contract_id);

    let milestones = vec![
        &env,
        Milestone {
            description: String::from_str(&env, "Only milestone"),
            amount: 100,
            status: MilestoneStatus::Pending,
        },
    ];
    let id = escrow_client.create_escrow(&client_addr, &provider, &arbiter, &token_address, &milestones);

    escrow_client.approve_milestone(&id, &0);
    escrow_client.approve_milestone(&id, &0); // should panic
}