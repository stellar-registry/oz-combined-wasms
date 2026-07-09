extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

fn create_client<'a>(e: &Env, owner: &Address) -> ContractClient<'a> {
    let name = String::from_str(e, "Test Token");
    let symbol = String::from_str(e, "TST");
    let address = e.register(Contract, (name, symbol, 18u32, owner, 1_000_000i128));
    ContractClient::new(e, &address)
}

#[test]
fn initial_state() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner);
    assert_eq!(client.total_supply(), 0);
    assert_eq!(client.symbol(), String::from_str(&e, "TST"));
    assert_eq!(client.name(), String::from_str(&e, "Test Token"));
    assert_eq!(client.decimals(), 18);
}

#[test]
fn transfer_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    let recipient = Address::generate(&e);
    client.transfer(&owner, &recipient, &100);
    assert_eq!(client.balance(&owner), 900);
    assert_eq!(client.balance(&recipient), 100);
}

#[test]
fn mint_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.mint(&owner, &500);
    assert_eq!(client.total_supply(), 1500);
}

#[test]
fn burn_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.burn(&owner, &200);
    assert_eq!(client.total_supply(), 800);
    assert_eq!(client.balance(&owner), 800);
}

#[test]
#[should_panic]
fn mint_exceeds_cap_fails() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.mint(&owner, &1_000_000);
}

