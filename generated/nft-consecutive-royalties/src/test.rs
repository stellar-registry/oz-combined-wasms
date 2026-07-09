extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

fn create_client<'a>(e: &Env, admin: &Address, manager: &Address) -> ContractClient<'a> {
    let uri = String::from_str(e, "https://example.com/");
    let name = String::from_str(e, "Test NFT");
    let symbol = String::from_str(e, "TNFT");
    let address = e.register(Contract, (uri, name, symbol, admin, manager, 1000u32));
    ContractClient::new(e, &address)
}

#[test]
fn mint_works() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &admin, &manager);
    let last_id = client.batch_mint(&user, &3);
    assert_eq!(client.balance(&user), 3);
    assert_eq!(last_id, 2);
}

#[test]
fn transfer_works() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let client = create_client(&e, &admin, &manager);
    client.batch_mint(&user1, &1);
    client.transfer(&user1, &user2, &0);
    assert_eq!(client.balance(&user1), 0);
    assert_eq!(client.balance(&user2), 1);
}

#[test]
fn burn_works() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &admin, &manager);
    client.batch_mint(&user, &1);
    client.burn(&user, &0);
    assert_eq!(client.balance(&user), 0);
}

#[test]
fn royalty_info_works() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let user = Address::generate(&e);
    let client = create_client(&e, &admin, &manager);
    client.batch_mint(&user, &1);
    let (receiver, amount) = client.royalty_info(&0, &10000);
    assert_eq!(receiver, admin);
    assert_eq!(amount, 1000); // 10% of 10000
}

