use crate::config::*;

/// Generate FungibleToken + FungibleBurnable impls (non-pausable, uses contracttrait defaults).
pub fn gen_fungible_default(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    let ct = match cfg.fungible_type().unwrap() {
        FungibleType::Base => "Base",
        FungibleType::AllowList => "AllowList",
        FungibleType::BlockList => "BlockList",
        FungibleType::Votes => "FungibleVotes",
        FungibleType::Vault => return gen_vault_default(),
    };

    // FungibleToken
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {ct};\n"));

    // Vault needs decimals override, handled separately
    out.push_str("}\n\n");

    // FungibleBurnable (not needed for Vault)
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleBurnable for Contract {}\n\n");

    out
}

fn gen_vault_default() -> String {
    let mut out = String::new();

    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleToken for Contract {\n");
    out.push_str("    type ContractType = Vault;\n\n");
    out.push_str("    fn decimals(e: &Env) -> u32 {\n");
    out.push_str("        Vault::decimals(e)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleBurnable for Contract {}\n\n");

    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleVault for Contract {}\n\n");

    out
}

/// Generate FungibleToken + FungibleBurnable impls (pausable, manual delegation with guards).
pub fn gen_fungible_pausable(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    let ct = match cfg.fungible_type().unwrap() {
        FungibleType::Base => "Base",
        FungibleType::AllowList => "AllowList",
        FungibleType::BlockList => "BlockList",
        FungibleType::Votes => "FungibleVotes",
        FungibleType::Vault => unreachable!("Vault + Pausable is not a valid combination"),
    };

    // Manual FungibleToken impl
    out.push_str("#[contractimpl]\n");
    out.push_str("impl FungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {ct};\n\n"));

    out.push_str("    fn total_supply(e: &Env) -> i128 {\n");
    out.push_str("        Self::ContractType::total_supply(e)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn balance(e: &Env, account: Address) -> i128 {\n");
    out.push_str("        Self::ContractType::balance(e, &account)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn allowance(e: &Env, owner: Address, spender: Address) -> i128 {\n");
    out.push_str("        Self::ContractType::allowance(e, &owner, &spender)\n");
    out.push_str("    }\n\n");

    out.push_str("    #[when_not_paused]\n");
    out.push_str(
        "    fn transfer(e: &Env, from: Address, to: MuxedAddress, amount: i128) {\n",
    );
    out.push_str("        Self::ContractType::transfer(e, &from, &to, amount);\n");
    out.push_str("    }\n\n");

    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {\n");
    out.push_str("        Self::ContractType::transfer_from(e, &spender, &from, &to, amount);\n");
    out.push_str("    }\n\n");

    out.push_str("    fn approve(e: &Env, owner: Address, spender: Address, amount: i128, live_until_ledger: u32) {\n");
    out.push_str("        Self::ContractType::approve(e, &owner, &spender, amount, live_until_ledger);\n");
    out.push_str("    }\n\n");

    out.push_str("    fn decimals(e: &Env) -> u32 {\n");
    out.push_str("        Self::ContractType::decimals(e)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn name(e: &Env) -> String {\n");
    out.push_str("        Self::ContractType::name(e)\n");
    out.push_str("    }\n\n");

    out.push_str("    fn symbol(e: &Env) -> String {\n");
    out.push_str("        Self::ContractType::symbol(e)\n");
    out.push_str("    }\n");

    out.push_str("}\n\n");

    // Manual FungibleBurnable impl
    out.push_str("#[contractimpl]\n");
    out.push_str("impl FungibleBurnable for Contract {\n");
    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn burn(e: &Env, from: Address, amount: i128) {\n");
    out.push_str("        Self::ContractType::burn(e, &from, amount)\n");
    out.push_str("    }\n\n");
    out.push_str("    #[when_not_paused]\n");
    out.push_str("    fn burn_from(e: &Env, spender: Address, from: Address, amount: i128) {\n");
    out.push_str("        Self::ContractType::burn_from(e, &spender, &from, amount)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}
