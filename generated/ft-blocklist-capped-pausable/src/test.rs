extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

fn create_client<'a>(e: &Env, admin: &Address, manager: &Address, initial_supply: i128) -> ContractClient<'a> {
    let name = String::from_str(e, "Test Token");
    let symbol = String::from_str(e, "TST");
    let address = e.register(Contract, (name, symbol, 18u32, admin, manager, 1_000_000i128, initial_supply));
    ContractClient::new(e, &address)
}

#[test]
fn initial_state() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    assert_eq!(client.total_supply(), 1000);
    assert_eq!(client.balance(&admin), 1000);
    assert_eq!(client.symbol(), String::from_str(&e, "TST"));
    assert_eq!(client.name(), String::from_str(&e, "Test Token"));
    assert_eq!(client.decimals(), 18);
    assert!(!client.paused());
}

#[test]
fn transfer_works() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    let recipient = Address::generate(&e);
    client.transfer(&admin, &recipient, &100);
    assert_eq!(client.balance(&admin), 900);
    assert_eq!(client.balance(&recipient), 100);
}

#[test]
fn mint_works() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    client.mint(&admin, &500);
    assert_eq!(client.total_supply(), 1500);
}

#[test]
fn burn_works() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    client.burn(&admin, &200);
    assert_eq!(client.total_supply(), 800);
    assert_eq!(client.balance(&admin), 800);
}

#[test]
#[should_panic(expected = "Error(Contract, #1000)")]
fn transfer_fails_when_paused() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    let recipient = Address::generate(&e);
    client.pause(&admin);
    client.transfer(&admin, &recipient, &100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1000)")]
fn mint_fails_when_paused() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    client.pause(&admin);
    client.mint(&admin, &500);
}

#[test]
#[should_panic(expected = "Error(Contract, #1000)")]
fn burn_fails_when_paused() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    client.pause(&admin);
    client.burn(&admin, &200);
}

#[test]
#[should_panic]
fn mint_exceeds_cap_fails() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    client.mint(&admin, &1_000_000);
}

#[test]
#[should_panic]
fn blocked_user_cannot_transfer() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &admin, &manager, 1000);
    e.mock_all_auths();
    // Transfer some tokens to user first
    client.transfer(&admin, &user, &500);
    // Block the user
    client.block_user(&user, &manager);
    // This should fail
    client.transfer(&user, &admin, &100);
}

