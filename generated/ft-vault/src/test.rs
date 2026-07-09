extern crate std;

use soroban_sdk::{
    contract, contractimpl, testutils::Address as _, Address, Env, MuxedAddress, String,
};
use stellar_tokens::fungible::{Base, FungibleToken};

use crate::contract::{Contract, ContractClient};

#[contract]
pub struct MockAssetContract;

#[contractimpl]
impl MockAssetContract {
    pub fn __constructor(e: &Env, initial_supply: i128, admin: Address) {
        Base::set_metadata(e, 18, String::from_str(e, "Mock Asset"), String::from_str(e, "MAT"));
        Base::mint(e, &admin, initial_supply);
    }
}

#[contractimpl(contracttrait)]
impl FungibleToken for MockAssetContract {
    type ContractType = stellar_tokens::fungible::Base;
}

fn create_vault_client<'a>(e: &Env, asset_address: &Address, owner: &Address) -> ContractClient<'a> {
    let name = String::from_str(e, "Vault Token");
    let symbol = String::from_str(e, "VLT");
    let address = e.register(Contract, (name, symbol, asset_address, 0u32, owner));
    ContractClient::new(e, &address)
}

fn create_asset_client<'a>(e: &Env, initial_supply: i128, admin: &Address) -> MockAssetContractClient<'a> {
    let address = e.register(MockAssetContract, (initial_supply, admin));
    MockAssetContractClient::new(e, &address)
}

#[test]
fn vault_deposit_works() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let user = Address::generate(&e);
    let asset_client = create_asset_client(&e, 1_000_000, &admin);
    let vault_client = create_vault_client(&e, &asset_client.address, &owner);
    asset_client.transfer(&admin, &user, &100_000);
    let shares = vault_client.deposit(&100_000, &user, &user, &user);
    assert!(shares > 0);
    assert_eq!(vault_client.total_assets(), 100_000);
}

#[test]
fn vault_withdraw_works() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let user = Address::generate(&e);
    let asset_client = create_asset_client(&e, 1_000_000, &admin);
    let vault_client = create_vault_client(&e, &asset_client.address, &owner);
    asset_client.transfer(&admin, &user, &100_000);
    vault_client.deposit(&100_000, &user, &user, &user);
    vault_client.withdraw(&50_000, &user, &user, &user);
    assert_eq!(asset_client.balance(&user), 50_000);
}

