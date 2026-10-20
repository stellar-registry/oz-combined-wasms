use crate::config::*;

pub fn contract_type(cfg: &CombinationConfig) -> String {
    let base = match cfg.nft_type().unwrap() {
        NftType::Base => "Base",
        NftType::Enumerable => "Enumerable",
        NftType::Consecutive => "Consecutive",
        NftType::Votes => "NonFungibleVotes",
    };
    if cfg.has(Feature::Royalties) {
        format!("Compose<({base}, Royalties)>")
    } else {
        format!("Compose<({base},)>")
    }
}

/// Generate NonFungibleToken + NonFungibleBurnable impls (non-pausable).
pub fn gen_nft_default(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    let ct = contract_type(cfg);

    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl NonFungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {ct};\n"));
    out.push_str("}\n\n");

    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl NonFungibleBurnable for Contract {}\n\n");

    // Extension trait impls for Enumerable/Consecutive
    match cfg.nft_type().unwrap() {
        NftType::Enumerable => {
            out.push_str("#[contractimpl(contracttrait)]\n");
            out.push_str("impl NonFungibleEnumerable for Contract {}\n\n");
        }
        NftType::Consecutive => {
            out.push_str("impl NonFungibleConsecutive for Contract {}\n\n");
        }
        _ => {}
    }

    out
}

/// Generate NonFungibleToken + NonFungibleBurnable impls (pausable, manual delegation).
pub fn gen_nft_pausable(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    let ct = contract_type(cfg);

    // Manual NonFungibleToken impl
    out.push_str("#[contractimpl]\n");
    out.push_str("impl NonFungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {ct};\n\n"));

    out.push_str("    fn balance(e: &Env, owner: Address) -> u32 {\n");
    out.push_str("        Self::ContractType::balance(e, &owner)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn owner_of(e: &Env, token_id: u32) -> Address {\n");
    out.push_str("        Self::ContractType::owner_of(e, token_id)\n");
    out.push_str("    }\n\n");

    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn transfer(e: &Env, from: Address, to: Address, token_id: u32) {\n");
    out.push_str("        Self::ContractType::transfer(e, &from, &to, token_id);\n");
    out.push_str("    }\n\n");

    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, token_id: u32) {\n");
    out.push_str(
        "        Self::ContractType::transfer_from(e, &spender, &from, &to, token_id);\n",
    );
    out.push_str("    }\n\n");

    out.push_str("    fn approve(e: &Env, approver: Address, approved: Address, token_id: u32, live_until_ledger: u32) {\n");
    out.push_str(
        "        Self::ContractType::approve(e, &approver, &approved, token_id, live_until_ledger);\n",
    );
    out.push_str("    }\n\n");

    out.push_str(
        "    fn approve_for_all(e: &Env, owner: Address, operator: Address, live_until_ledger: u32) {\n",
    );
    out.push_str(
        "        Self::ContractType::approve_for_all(e, &owner, &operator, live_until_ledger);\n",
    );
    out.push_str("    }\n\n");

    out.push_str("    fn get_approved(e: &Env, token_id: u32) -> Option<Address> {\n");
    out.push_str("        Self::ContractType::get_approved(e, token_id)\n");
    out.push_str("    }\n\n");

    out.push_str(
        "    fn is_approved_for_all(e: &Env, owner: Address, operator: Address) -> bool {\n",
    );
    out.push_str("        Self::ContractType::is_approved_for_all(e, &owner, &operator)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn name(e: &Env) -> String {\n");
    out.push_str("        Self::ContractType::name(e)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn symbol(e: &Env) -> String {\n");
    out.push_str("        Self::ContractType::symbol(e)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn token_uri(e: &Env, token_id: u32) -> String {\n");
    out.push_str("        Self::ContractType::token_uri(e, token_id)\n");
    out.push_str("    }\n");

    out.push_str("}\n\n");

    // Manual NonFungibleBurnable impl
    out.push_str("#[contractimpl]\n");
    out.push_str("impl NonFungibleBurnable for Contract {\n");
    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn burn(e: &Env, from: Address, token_id: u32) {\n");
    out.push_str("        Self::ContractType::burn(e, &from, token_id)\n");
    out.push_str("    }\n\n");
    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn burn_from(e: &Env, spender: Address, from: Address, token_id: u32) {\n");
    out.push_str("        Self::ContractType::burn_from(e, &spender, &from, token_id)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Extension trait impls for Enumerable/Consecutive
    match cfg.nft_type().unwrap() {
        NftType::Enumerable => {
            out.push_str("#[contractimpl(contracttrait)]\n");
            out.push_str("impl NonFungibleEnumerable for Contract {}\n\n");
        }
        NftType::Consecutive => {
            out.push_str("impl NonFungibleConsecutive for Contract {}\n\n");
        }
        _ => {}
    }

    out
}
