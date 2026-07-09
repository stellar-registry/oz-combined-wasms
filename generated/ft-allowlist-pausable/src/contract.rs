//! Pausable SEP-41 fungible token with AllowList, AccessControl, Burnable, and Upgradeable.

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, MuxedAddress, String, Symbol, Vec};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};
use stellar_macros::{only_admin, only_role, when_not_paused};
use stellar_tokens::fungible::{burnable::FungibleBurnable, allowlist::{AllowList, FungibleAllowList}, ContractOverrides, Base, FungibleToken};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(
        e: &Env,
        name: String,
        symbol: String,
        decimals: u32,
        admin: Address,
        manager: Address,
        initial_supply: i128,
    ) {
        Base::set_metadata(e, decimals, name, symbol);
        access_control::set_admin(e, &admin);
        access_control::grant_role_no_auth(e, &manager, &symbol_short!("manager"), &admin);
        AllowList::allow_user(e, &admin);
        Base::mint(e, &admin, initial_supply);
    }

    #[only_admin]
    #[when_not_paused]
    pub fn mint(e: &Env, to: Address, amount: i128) {
        Base::mint(e, &to, amount);
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
impl FungibleToken for Contract {
    type ContractType = AllowList;

    fn total_supply(e: &Env) -> i128 {
        Self::ContractType::total_supply(e)
    }

    fn balance(e: &Env, account: Address) -> i128 {
        Self::ContractType::balance(e, &account)
    }

    fn allowance(e: &Env, owner: Address, spender: Address) -> i128 {
        Self::ContractType::allowance(e, &owner, &spender)
    }

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: MuxedAddress, amount: i128) {
        Self::ContractType::transfer(e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        Self::ContractType::transfer_from(e, &spender, &from, &to, amount);
    }

    fn approve(e: &Env, owner: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        Self::ContractType::approve(e, &owner, &spender, amount, live_until_ledger);
    }

    fn decimals(e: &Env) -> u32 {
        Self::ContractType::decimals(e)
    }

    fn name(e: &Env) -> String {
        Self::ContractType::name(e)
    }

    fn symbol(e: &Env) -> String {
        Self::ContractType::symbol(e)
    }
}

#[contractimpl]
impl FungibleBurnable for Contract {
    #[when_not_paused]
    fn burn(e: &Env, from: Address, amount: i128) {
        Self::ContractType::burn(e, &from, amount)
    }

    #[when_not_paused]
    fn burn_from(e: &Env, spender: Address, from: Address, amount: i128) {
        Self::ContractType::burn_from(e, &spender, &from, amount)
    }
}

#[contractimpl(contracttrait)]
impl FungibleAllowList for Contract {
    #[only_role(operator, "manager")]
    fn allow_user(e: &Env, user: Address, operator: Address) {
        AllowList::allow_user(e, &user)
    }

    #[only_role(operator, "manager")]
    fn disallow_user(e: &Env, user: Address, operator: Address) {
        AllowList::disallow_user(e, &user)
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
