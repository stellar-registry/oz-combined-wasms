extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

fn create_client<'a>(e: &Env, owner: &Address) -> ContractClient<'a> {
    let uri = String::from_str(e, "https://example.com/");
    let name = String::from_str(e, "Test NFT");
    let symbol = String::from_str(e, "TNFT");
    let address = e.register(Contract, (uri, name, symbol, owner));
    ContractClient::new(e, &address)
}

#[test]
fn mint_works() {
    let e = Env::default();
    e.mock_all_auths();
    let owner = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &owner);
    let token_id = client.mint(&user);
    assert_eq!(client.balance(&user), 1);
    assert_eq!(client.owner_of(&token_id), user);
}

#[test]
fn transfer_works() {
    let e = Env::default();
    e.mock_all_auths();
    let owner = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let client = create_client(&e, &owner);
    let token_id = client.mint(&user1);
    client.transfer(&user1, &user2, &token_id);
    assert_eq!(client.balance(&user1), 0);
    assert_eq!(client.balance(&user2), 1);
}

#[test]
fn burn_works() {
    let e = Env::default();
    e.mock_all_auths();
    let owner = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &owner);
    let token_id = client.mint(&user);
    client.burn(&user, &token_id);
    assert_eq!(client.balance(&user), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #1000)")]
fn transfer_fails_when_paused() {
    let e = Env::default();
    e.mock_all_auths();
    let owner = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let client = create_client(&e, &owner);
    let token_id = client.mint(&user1);
    client.pause(&owner);
    client.transfer(&user1, &user2, &token_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #1000)")]
fn mint_fails_when_paused() {
    let e = Env::default();
    e.mock_all_auths();
    let owner = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &owner);
    client.pause(&owner);
    client.mint(&user);
}

