//! Capped SEP-41 fungible token with BlockList, AccessControl, Burnable, and Upgradeable.

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, MuxedAddress, String, Symbol, Vec};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};
use stellar_macros::{only_admin, only_role};
use stellar_tokens::fungible::{burnable::FungibleBurnable, capped::{Capped, FungibleCapped}, blocklist::{BlockList, FungibleBlockList}, total_supply::{FungibleTotalSupply, TotalSupply}, Base, Compose, FungibleToken};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    #[allow(clippy::too_many_arguments)]
    pub fn __constructor(
        e: &Env,
        name: String,
        symbol: String,
        decimals: u32,
        admin: Address,
        manager: Address,
        cap: i128,
        initial_supply: i128,
    ) {
        Base::set_metadata(e, decimals, name, symbol);
        access_control::set_admin(e, &admin);
        access_control::grant_role_no_auth(e, &manager, &symbol_short!("manager"), &admin);
        Capped::set_cap(e, cap);
        <Self as FungibleToken>::ContractType::mint(e, &admin, initial_supply);
    }

    #[only_admin]
    pub fn mint(e: &Env, to: Address, amount: i128) {
        <Self as FungibleToken>::ContractType::mint(e, &to, amount);
    }
}

#[contractimpl(contracttrait)]
impl FungibleToken for Contract {
    type ContractType = Compose<(BlockList, Capped, TotalSupply)>;
}

#[contractimpl(contracttrait)]
impl FungibleBurnable for Contract {}

#[contractimpl(contracttrait)]
impl FungibleTotalSupply for Contract {}

#[contractimpl(contracttrait)]
impl FungibleCapped for Contract {}

#[contractimpl(contracttrait)]
impl FungibleBlockList for Contract {
    #[only_role(operator, "manager")]
    fn block_user(e: &Env, user: Address, operator: Address) {
        BlockList::block_user(e, &user)
    }

    #[only_role(operator, "manager")]
    fn unblock_user(e: &Env, user: Address, operator: Address) {
        BlockList::unblock_user(e, &user)
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
