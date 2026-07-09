//! Consecutive NFT with royalties, AccessControl, Burnable, and Upgradeable.

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, String, Symbol, Vec};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};
use stellar_macros::{only_admin, only_role};
use stellar_tokens::non_fungible::{burnable::NonFungibleBurnable, consecutive::{Consecutive, NonFungibleConsecutive}, royalties::NonFungibleRoyalties, Base, NonFungibleToken};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(
        e: &Env,
        uri: String,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
        default_royalty_bps: u32,
    ) {
        Base::set_metadata(e, uri, name, symbol);
        Base::set_default_royalty(e, &admin, default_royalty_bps);
        access_control::set_admin(e, &admin);
        access_control::grant_role_no_auth(e, &manager, &symbol_short!("manager"), &admin);
    }

    #[only_admin]
    pub fn batch_mint(e: &Env, to: Address, amount: u32) -> u32 {
        Consecutive::batch_mint(e, &to, amount)
    }
}

#[contractimpl(contracttrait)]
impl NonFungibleToken for Contract {
    type ContractType = Consecutive;
}

#[contractimpl(contracttrait)]
impl NonFungibleBurnable for Contract {}

impl NonFungibleConsecutive for Contract {}

#[contractimpl(contracttrait)]
impl NonFungibleRoyalties for Contract {
    #[only_role(operator, "manager")]
    fn set_default_royalty(e: &Env, receiver: Address, basis_points: u32, operator: Address) {
        Base::set_default_royalty(e, &receiver, basis_points);
    }

    #[only_role(operator, "manager")]
    fn set_token_royalty(e: &Env, token_id: u32, receiver: Address, basis_points: u32, operator: Address) {
        Base::set_token_royalty(e, token_id, &receiver, basis_points);
    }

    #[only_role(operator, "manager")]
    fn remove_token_royalty(e: &Env, token_id: u32, operator: Address) {
        Base::remove_token_royalty(e, token_id);
    }
}

#[contractimpl(contracttrait)]
impl AccessControl for Contract {}

#[contractimpl]
#[allow(unused_variables)]
impl Upgradeable for Contract {
    #[only_admin]
    fn upgrade(e: &Env, new_wasm_hash: BytesN<32>, operator: Address) {
        upgradeable::upgrade(e, &new_wasm_hash);
    }
}
