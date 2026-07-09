//! Standard SEP-41 fungible token with Ownable access, Burnable, and Upgradeable.

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, MuxedAddress, String};
use stellar_access::ownable::{set_owner, Ownable};
use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};
use stellar_macros::{only_owner};
use stellar_tokens::fungible::{burnable::FungibleBurnable, Base, FungibleToken};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(
        e: &Env,
        name: String,
        symbol: String,
        decimals: u32,
        owner: Address,
        initial_supply: i128,
    ) {
        Base::set_metadata(e, decimals, name, symbol);
        set_owner(e, &owner);
        Base::mint(e, &owner, initial_supply);
    }

    #[only_owner]
    pub fn mint(e: &Env, to: Address, amount: i128) {
        Base::mint(e, &to, amount);
    }
}

#[contractimpl(contracttrait)]
impl FungibleToken for Contract {
    type ContractType = Base;
}

#[contractimpl(contracttrait)]
impl FungibleBurnable for Contract {}

#[contractimpl(contracttrait)]
impl Ownable for Contract {}

#[contractimpl]
#[allow(unused_variables)]
impl Upgradeable for Contract {
    #[only_owner]
    fn upgrade(e: &Env, new_wasm_hash: BytesN<32>, operator: Address) {
        upgradeable::upgrade(e, &new_wasm_hash);
    }
}
