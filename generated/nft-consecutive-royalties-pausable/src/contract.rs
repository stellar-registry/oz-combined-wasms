//! Pausable consecutive NFT with royalties, AccessControl, Burnable, and Upgradeable.

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, String, Symbol, Vec};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};
use stellar_macros::{only_admin, only_role, when_not_paused};
use stellar_tokens::non_fungible::{burnable::NonFungibleBurnable, consecutive::{Consecutive, NonFungibleConsecutive}, royalties::NonFungibleRoyalties, ContractOverrides, Base, NonFungibleToken};

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
    #[when_not_paused]
    pub fn batch_mint(e: &Env, to: Address, amount: u32) -> u32 {
        Consecutive::batch_mint(e, &to, amount)
    }
}

#[contractimpl]
#[allow(unused_variables)]
impl Pausable for Contract {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    #[only_admin]
    fn pause(e: &Env, caller: Address) {
        pausable::pause(e);
    }

    #[only_admin]
    fn unpause(e: &Env, caller: Address) {
        pausable::unpause(e);
    }
}

#[contractimpl]
impl NonFungibleToken for Contract {
    type ContractType = Consecutive;

    fn balance(e: &Env, owner: Address) -> u32 {
        Self::ContractType::balance(e, &owner)
    }

    fn owner_of(e: &Env, token_id: u32) -> Address {
        Self::ContractType::owner_of(e, token_id)
    }

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: Address, token_id: u32) {
        Self::ContractType::transfer(e, &from, &to, token_id);
    }

    #[when_not_paused]
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, token_id: u32) {
        Self::ContractType::transfer_from(e, &spender, &from, &to, token_id);
    }

    fn approve(e: &Env, approver: Address, approved: Address, token_id: u32, live_until_ledger: u32) {
        Self::ContractType::approve(e, &approver, &approved, token_id, live_until_ledger);
    }

    fn approve_for_all(e: &Env, owner: Address, operator: Address, live_until_ledger: u32) {
        Self::ContractType::approve_for_all(e, &owner, &operator, live_until_ledger);
    }

    fn get_approved(e: &Env, token_id: u32) -> Option<Address> {
        Self::ContractType::get_approved(e, token_id)
    }

    fn is_approved_for_all(e: &Env, owner: Address, operator: Address) -> bool {
        Self::ContractType::is_approved_for_all(e, &owner, &operator)
    }

    fn name(e: &Env) -> String {
        Self::ContractType::name(e)
    }

    fn symbol(e: &Env) -> String {
        Self::ContractType::symbol(e)
    }

    fn token_uri(e: &Env, token_id: u32) -> String {
        Self::ContractType::token_uri(e, token_id)
    }
}

#[contractimpl]
impl NonFungibleBurnable for Contract {
    #[when_not_paused]
    fn burn(e: &Env, from: Address, token_id: u32) {
        Self::ContractType::burn(e, &from, token_id)
    }

    #[when_not_paused]
    fn burn_from(e: &Env, spender: Address, from: Address, token_id: u32) {
        Self::ContractType::burn_from(e, &spender, &from, token_id)
    }
}

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
