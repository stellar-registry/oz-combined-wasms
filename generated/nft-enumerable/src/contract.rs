//! Enumerable NFT with on-chain token enumeration, Ownable access, Burnable, and Upgradeable.

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};
use stellar_access::ownable::{set_owner, Ownable};
use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};
use stellar_macros::{only_owner};
use stellar_tokens::non_fungible::{burnable::NonFungibleBurnable, enumerable::{Enumerable, NonFungibleEnumerable}, Base, NonFungibleToken};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(
        e: &Env,
        uri: String,
        name: String,
        symbol: String,
        owner: Address,
    ) {
        Base::set_metadata(e, uri, name, symbol);
        set_owner(e, &owner);
    }

    #[only_owner]
    pub fn mint(e: &Env, to: Address) -> u32 {
        Enumerable::sequential_mint(e, &to)
    }
}

#[contractimpl(contracttrait)]
impl NonFungibleToken for Contract {
    type ContractType = Enumerable;
}

#[contractimpl(contracttrait)]
impl NonFungibleBurnable for Contract {}

#[contractimpl(contracttrait)]
impl NonFungibleEnumerable for Contract {}

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
