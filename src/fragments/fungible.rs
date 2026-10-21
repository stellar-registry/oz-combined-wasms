use crate::config::*;

/// `FungibleVotes` has no curated combination with `TotalSupply` or `Capped`,
/// so votes tokens track supply through their voting checkpoints instead.
pub fn tracks_total_supply(ft: FungibleType) -> bool {
    ft != FungibleType::Votes
}

pub fn contract_type(cfg: &CombinationConfig) -> String {
    let ft = cfg.fungible_type().unwrap();
    let mut entries = match ft {
        FungibleType::Base => vec![],
        FungibleType::AllowList => vec!["AllowList"],
        FungibleType::BlockList => vec!["BlockList"],
        FungibleType::Votes => vec!["FungibleVotes"],
        FungibleType::Vault => vec!["Vault"],
    };
    if tracks_total_supply(ft) {
        if cfg.has(Feature::Capped) {
            entries.push("Capped");
        }
        entries.push("TotalSupply");
    }
    if entries.len() == 1 {
        format!("Compose<({},)>", entries[0])
    } else {
        format!("Compose<({})>", entries.join(", "))
    }
}

fn gen_extension_views(out: &mut String, cfg: &CombinationConfig) {
    if !tracks_total_supply(cfg.fungible_type().unwrap()) {
        return;
    }
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleTotalSupply for Contract {}\n\n");
    if cfg.has(Feature::Capped) {
        out.push_str("#[contractimpl(contracttrait)]\n");
        out.push_str("impl FungibleCapped for Contract {}\n\n");
    }
}

/// Generate FungibleToken + FungibleBurnable impls (non-pausable, uses contracttrait defaults).
pub fn gen_fungible_default(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    if cfg.fungible_type() == Some(FungibleType::Vault) {
        return gen_vault_default(cfg);
    }
    let ct = contract_type(cfg);

    // FungibleToken
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {ct};\n"));

    // Vault needs decimals override, handled separately
    out.push_str("}\n\n");

    // FungibleBurnable (not needed for Vault)
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleBurnable for Contract {}\n\n");

    gen_extension_views(&mut out, cfg);

    out
}

fn gen_vault_default(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {};\n\n", contract_type(cfg)));
    out.push_str("    fn decimals(e: &Env) -> u32 {\n");
    out.push_str("        Vault::decimals(e)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Vault shares are redeemed via withdraw(), not burned directly
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleVault for Contract {}\n\n");

    gen_extension_views(&mut out, cfg);

    out
}

/// Generate FungibleToken + FungibleBurnable impls (pausable, manual delegation with guards).
pub fn gen_fungible_pausable(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    assert!(
        cfg.fungible_type() != Some(FungibleType::Vault),
        "Vault + Pausable is not a valid combination"
    );
    let ct = contract_type(cfg);

    // Manual FungibleToken impl
    out.push_str("#[contractimpl]\n");
    out.push_str("impl FungibleToken for Contract {\n");
    out.push_str(&format!("    type ContractType = {ct};\n\n"));

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

    gen_extension_views(&mut out, cfg);

    out
}
